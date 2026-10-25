//! The unit as a PipeWire sink and source ("Fireface UCX"), driven by the unit's own clock.
//!
//! Two `pw_filter`s with one float port per channel, in one node group, both able to drive; the
//! sink's driver priority is higher, so it drives. That is how PipeWire's own JACK tunnel bridges
//! an outside clock (`src/modules/module-jack-tunnel.c`). The streaming thread starts each graph
//! cycle itself as soon as the cycle's frames have been recorded: it fills in the graph clock and
//! calls `pw_filter_trigger_process`. So the graph runs at the unit's rate with nothing to
//! resample, and both directions use the unit's own positions: a cycle that records positions
//! P .. P + n plays what the graph made of it at P + `play_delay` .. + n.
//!
//! libpipewire is loaded at run time, so only the runtime library is needed (no headers). The
//! few structs used are copied from the PipeWire 1.6 headers.

use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicPtr, AtomicU8, Ordering};
use std::sync::{Arc, OnceLock};

use ucx_protocol::iso::InPacket;

use crate::engine::{Client, Stats};
use ucx_usb::now_ns;

type Ptr = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct Fraction {
    num: u32,
    denom: u32,
}

/// `struct spa_io_clock` (spa/node/io.h), the start of `struct spa_io_position`.
#[repr(C)]
struct Clock {
    flags: u32,
    id: u32,
    name: [c_char; 64],
    nsec: u64,
    rate: Fraction,
    position: u64,
    duration: u64,
    delay: i64,
    rate_diff: f64,
    next_nsec: u64,
    target_rate: Fraction,
    target_duration: u64,
    target_seq: u32,
    cycle: u32,
    xrun: u64,
}

const _: () = assert!(size_of::<Clock>() == 160);

/// `struct pw_filter_events`, version 1.
#[repr(C)]
struct FilterEvents {
    version: u32,
    destroy: Option<unsafe extern "C" fn(Ptr)>,
    state_changed: Option<unsafe extern "C" fn(Ptr, c_int, c_int, *const c_char)>,
    io_changed: Option<unsafe extern "C" fn(Ptr, Ptr, u32, Ptr, u32)>,
    param_changed: Option<unsafe extern "C" fn(Ptr, Ptr, u32, *const c_void)>,
    add_buffer: Option<unsafe extern "C" fn(Ptr, Ptr, Ptr)>,
    remove_buffer: Option<unsafe extern "C" fn(Ptr, Ptr, Ptr)>,
    process: Option<unsafe extern "C" fn(Ptr, *mut Clock)>,
    drained: Option<unsafe extern "C" fn(Ptr)>,
    command: Option<unsafe extern "C" fn(Ptr, *const c_void)>,
}

const SPA_IO_POSITION: u32 = 7;
const DIRECTION_INPUT: u32 = 0;
const DIRECTION_OUTPUT: u32 = 1;
const FILTER_FLAG_DRIVER: u32 = 1 << 1;
const FILTER_FLAG_RT_PROCESS: u32 = 1 << 2;
const PORT_FLAG_MAP_BUFFERS: u32 = 1 << 0;
const STATE_STREAMING: c_int = 3;

/// The libpipewire functions used.
struct Pw {
    init: unsafe extern "C" fn(*mut c_int, *mut c_void),
    deinit: unsafe extern "C" fn(),
    thread_loop_new: unsafe extern "C" fn(*const c_char, *const c_void) -> Ptr,
    thread_loop_get_loop: unsafe extern "C" fn(Ptr) -> Ptr,
    thread_loop_start: unsafe extern "C" fn(Ptr) -> c_int,
    thread_loop_stop: unsafe extern "C" fn(Ptr),
    thread_loop_lock: unsafe extern "C" fn(Ptr),
    thread_loop_unlock: unsafe extern "C" fn(Ptr),
    thread_loop_destroy: unsafe extern "C" fn(Ptr),
    context_new: unsafe extern "C" fn(Ptr, Ptr, usize) -> Ptr,
    context_destroy: unsafe extern "C" fn(Ptr),
    context_connect: unsafe extern "C" fn(Ptr, Ptr, usize) -> Ptr,
    core_disconnect: unsafe extern "C" fn(Ptr) -> c_int,
    properties_new_string: unsafe extern "C" fn(*const c_char) -> Ptr,
    filter_new: unsafe extern "C" fn(Ptr, *const c_char, Ptr) -> Ptr,
    filter_add_listener: unsafe extern "C" fn(Ptr, Ptr, *const FilterEvents, Ptr),
    filter_add_port: unsafe extern "C" fn(Ptr, u32, u32, usize, Ptr, *const c_void, u32) -> Ptr,
    filter_connect: unsafe extern "C" fn(Ptr, u32, *const c_void, u32) -> c_int,
    filter_update_params: unsafe extern "C" fn(Ptr, Ptr, *const c_void, u32) -> c_int,
    filter_destroy: unsafe extern "C" fn(Ptr),
    filter_get_dsp_buffer: unsafe extern "C" fn(Ptr, u32) -> *mut f32,
    filter_trigger_process: unsafe extern "C" fn(Ptr) -> c_int,
    filter_is_driving: unsafe extern "C" fn(Ptr) -> bool,
    filter_get_node_id: unsafe extern "C" fn(Ptr) -> u32,
    filter_state_as_string: unsafe extern "C" fn(c_int) -> *const c_char,
}

impl Pw {
    // each symbol becomes the type of the field it goes into
    #[allow(clippy::missing_transmute_annotations)]
    fn load() -> Result<&'static Pw, String> {
        let lib = unsafe { libc::dlopen(c"libpipewire-0.3.so.0".as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
        if lib.is_null() {
            let e = unsafe { CStr::from_ptr(libc::dlerror()) };
            return Err(format!("can't load libpipewire: {}", e.to_string_lossy()));
        }
        macro_rules! sym {
            ($name:literal) => {{
                let p = unsafe { libc::dlsym(lib, concat!($name, "\0").as_ptr().cast()) };
                if p.is_null() {
                    return Err(format!("{} missing from libpipewire", $name));
                }
                unsafe { std::mem::transmute::<*mut c_void, _>(p) }
            }};
        }
        let pw = Pw {
            init: sym!("pw_init"),
            deinit: sym!("pw_deinit"),
            thread_loop_new: sym!("pw_thread_loop_new"),
            thread_loop_get_loop: sym!("pw_thread_loop_get_loop"),
            thread_loop_start: sym!("pw_thread_loop_start"),
            thread_loop_stop: sym!("pw_thread_loop_stop"),
            thread_loop_lock: sym!("pw_thread_loop_lock"),
            thread_loop_unlock: sym!("pw_thread_loop_unlock"),
            thread_loop_destroy: sym!("pw_thread_loop_destroy"),
            context_new: sym!("pw_context_new"),
            context_destroy: sym!("pw_context_destroy"),
            context_connect: sym!("pw_context_connect"),
            core_disconnect: sym!("pw_core_disconnect"),
            properties_new_string: sym!("pw_properties_new_string"),
            filter_new: sym!("pw_filter_new"),
            filter_add_listener: sym!("pw_filter_add_listener"),
            filter_add_port: sym!("pw_filter_add_port"),
            filter_connect: sym!("pw_filter_connect"),
            filter_update_params: sym!("pw_filter_update_params"),
            filter_destroy: sym!("pw_filter_destroy"),
            filter_get_dsp_buffer: sym!("pw_filter_get_dsp_buffer"),
            filter_trigger_process: sym!("pw_filter_trigger_process"),
            filter_is_driving: sym!("pw_filter_is_driving"),
            filter_get_node_id: sym!("pw_filter_get_node_id"),
            filter_state_as_string: sym!("pw_filter_state_as_string"),
        };
        Ok(Box::leak(Box::new(pw)))
    }

    fn properties(&self, s: &str) -> Ptr {
        let s = CString::new(s).unwrap();
        unsafe { (self.properties_new_string)(s.as_ptr()) }
    }
}

/// Frames of every channel, interleaved, indexed by the unit's position modulo the length. One
/// thread writes, one reads; positions published through atomics keep them apart.
struct Ring {
    frames: usize,
    channels: usize,
    data: Box<[UnsafeCell<f32>]>,
}

unsafe impl Sync for Ring {}

impl Ring {
    fn new(frames: usize, channels: usize) -> Ring {
        assert!(frames.is_power_of_two());
        Ring { frames, channels, data: (0..frames * channels).map(|_| UnsafeCell::new(0.0)).collect() }
    }

    fn slot(&self, position: i64, channel: usize) -> *mut f32 {
        self.data[(position as usize & (self.frames - 1)) * self.channels + channel].get()
    }
}

const SINK: u8 = 1;
const SOURCE: u8 = 2;
const RING_FRAMES: usize = 1 << 15;
/// Position recorded first, never played.
const NOWHERE: i64 = i64::MIN;

/// One of the two filters, as the callbacks see it.
struct Node {
    filter: AtomicPtr<c_void>,
    ports: OnceLock<Vec<usize>>,
    position: AtomicPtr<Clock>,
    state: AtomicI32,
    /// `struct spa_hook`: six pointers.
    hook: UnsafeCell<[usize; 6]>,
}

impl Node {
    fn new() -> Node {
        Node {
            filter: AtomicPtr::new(std::ptr::null_mut()),
            ports: OnceLock::new(),
            position: AtomicPtr::new(std::ptr::null_mut()),
            state: AtomicI32::new(0),
            hook: UnsafeCell::new([0; 6]),
        }
    }
}

/// What the streaming thread and PipeWire's data thread share.
struct Shared {
    pw: &'static Pw,
    capture: Ring,
    playback: Ring,
    play_delay: i64,
    sink: Node,
    source: Node,
    /// The running cycle's first recorded position.
    cycle_pos: AtomicI64,
    /// The node driving the running cycle (SINK or SOURCE); 0 between cycles.
    running: AtomicU8,
    /// The driver's own call at the start of the running cycle hasn't come yet.
    start_pending: AtomicBool,
    /// The playback ring holds frames up to here (exclusive).
    played_to: AtomicI64,
}

unsafe impl Sync for Shared {}
unsafe impl Send for Shared {}

impl Shared {
    fn node(&self, which: u8) -> &Node {
        if which == SINK {
            &self.sink
        } else {
            &self.source
        }
    }

    /// The source's ports get the cycle's recorded frames.
    unsafe fn produce(&self, n: usize) {
        let p = self.cycle_pos.load(Ordering::Acquire);
        for (c, &port) in self.source.ports.get().into_iter().flatten().enumerate() {
            let buf = (self.pw.filter_get_dsp_buffer)(port as Ptr, n as u32);
            if !buf.is_null() {
                for f in 0..n {
                    *buf.add(f) = *self.capture.slot(p + f as i64, c);
                }
            }
        }
    }

    /// The sink's ports go into the playback ring, `play_delay` after the cycle's positions.
    unsafe fn consume(&self, n: usize) {
        let p = self.cycle_pos.load(Ordering::Acquire) + self.play_delay;
        for (c, &port) in self.sink.ports.get().into_iter().flatten().enumerate() {
            // unlinked ports have no buffer: silence
            let buf = (self.pw.filter_get_dsp_buffer)(port as Ptr, n as u32);
            for f in 0..n {
                *self.playback.slot(p + f as i64, c) = if buf.is_null() { 0.0 } else { *buf.add(f) };
            }
        }
        self.played_to.store(p + n as i64, Ordering::Release);
    }
}

unsafe extern "C" fn on_process<const WHICH: u8>(data: Ptr, position: *mut Clock) {
    let sh = &*(data as *const Shared);
    let driver = sh.running.load(Ordering::Acquire);
    if driver == 0 || position.is_null() {
        return; // not a cycle the stream started
    }
    let n = (*position).duration as usize;
    if driver == WHICH {
        // the driver is called at the start of the cycle and again at its end
        if sh.start_pending.swap(false, Ordering::AcqRel) {
            if WHICH == SOURCE {
                sh.produce(n);
            }
            return;
        }
        if WHICH == SINK {
            sh.consume(n);
        }
        sh.running.store(0, Ordering::Release);
    } else if WHICH == SINK {
        sh.consume(n);
    } else {
        sh.produce(n);
    }
}

unsafe extern "C" fn on_io_changed<const WHICH: u8>(data: Ptr, port: Ptr, id: u32, area: Ptr, _size: u32) {
    let sh = &*(data as *const Shared);
    if port.is_null() && id == SPA_IO_POSITION {
        sh.node(WHICH).position.store(area.cast(), Ordering::Release);
    }
}

/// The session manager sets PortConfig before it links a node, and waits for the node's ports
/// to change or, when they are there already, for its Props to be emitted again
/// (`module-si-audio-adapter.c`, `on_node_params_changed`). The ports stay as they are.
unsafe extern "C" fn on_param_changed<const WHICH: u8>(data: Ptr, port: Ptr, id: u32, _param: *const c_void) {
    let sh = &*(data as *const Shared);
    let filter = sh.node(WHICH).filter.load(Ordering::Acquire);
    if port.is_null() && id == PARAM_PORT_CONFIG && !filter.is_null() {
        let props = props_pod(sh.capture.channels);
        let params = [props.as_ptr() as *const c_void];
        (sh.pw.filter_update_params)(filter, std::ptr::null_mut(), params.as_ptr().cast(), 1);
    }
}

unsafe extern "C" fn on_state_changed<const WHICH: u8>(data: Ptr, _old: c_int, state: c_int, error: *const c_char) {
    let sh = &*(data as *const Shared);
    sh.node(WHICH).state.store(state, Ordering::Release);
    let name = CStr::from_ptr((sh.pw.filter_state_as_string)(state)).to_string_lossy();
    let error = if error.is_null() { String::new() } else { format!(": {}", CStr::from_ptr(error).to_string_lossy()) };
    let filter = sh.node(WHICH).filter.load(Ordering::Acquire);
    let id = if filter.is_null() { u32::MAX } else { (sh.pw.filter_get_node_id)(filter) };
    let id = if id == u32::MAX { String::new() } else { format!(" (node {id})") };
    eprintln!("PipeWire {}{id}: {name}{error}", if WHICH == SINK { "sink" } else { "source" });
}

static SINK_EVENTS: FilterEvents = events::<SINK>();
static SOURCE_EVENTS: FilterEvents = events::<SOURCE>();

const fn events<const WHICH: u8>() -> FilterEvents {
    FilterEvents {
        version: 1,
        destroy: None,
        state_changed: Some(on_state_changed::<WHICH>),
        io_changed: Some(on_io_changed::<WHICH>),
        param_changed: Some(on_param_changed::<WHICH>),
        add_buffer: None,
        remove_buffer: None,
        process: Some(on_process::<WHICH>),
        drained: None,
        command: None,
    }
}

/// Port names, `audio.channel`s and SPA channel positions in DSP channel order (analog 1–8,
/// SPDIF L/R, ADAT). Channels `main` and `main` + 1 (0-based) are FL/FR, so stereo streams land
/// there; the others are AUXn, n being their index.
fn channel_names(channels: usize, main: usize) -> Vec<(String, String, u32)> {
    let names = (1..=8).map(|i| format!("AN{i}")).chain(["SPDIF_L".into(), "SPDIF_R".into()]);
    let names = names.chain((1..=8).map(|i| format!("ADAT{i}")));
    names
        .take(channels)
        .enumerate()
        .map(|(i, n)| match i {
            _ if i == main => (n, "FL".into(), CHANNEL_FL),
            _ if i == main + 1 => (n, "FR".into(), CHANNEL_FR),
            _ => (n, format!("AUX{i}"), CHANNEL_AUX0 + i as u32),
        })
        .collect()
}

// spa/utils/type.h, spa/param/param.h, spa/param/format.h, spa/param/audio/raw.h
const TYPE_BOOL: u32 = 2;
const TYPE_ID: u32 = 3;
const TYPE_INT: u32 = 4;
const TYPE_LONG: u32 = 5;
const TYPE_FLOAT: u32 = 6;
const TYPE_ARRAY: u32 = 13;
const TYPE_OBJECT: u32 = 15;
const TYPE_OBJECT_PROPS: u32 = 0x40002;
const TYPE_OBJECT_FORMAT: u32 = 0x40003;
const TYPE_OBJECT_PARAM_LATENCY: u32 = 0x4000b;
const PARAM_PROPS: u32 = 2;
const PARAM_ENUM_FORMAT: u32 = 3;
const PARAM_FORMAT: u32 = 4;
const PARAM_PORT_CONFIG: u32 = 11;
const PARAM_LATENCY: u32 = 15;
const LATENCY_DIRECTION: u32 = 1;
const LATENCY_MIN_QUANTUM: u32 = 2;
const LATENCY_MAX_QUANTUM: u32 = 3;
const LATENCY_MIN_RATE: u32 = 4;
const LATENCY_MAX_RATE: u32 = 5;
const LATENCY_MIN_NS: u32 = 6;
const LATENCY_MAX_NS: u32 = 7;
const PROP_MUTE: u32 = 0x10004;
const PROP_CHANNEL_VOLUMES: u32 = 0x10008;
const FORMAT_MEDIA_TYPE: u32 = 1;
const FORMAT_MEDIA_SUBTYPE: u32 = 2;
const FORMAT_AUDIO_FORMAT: u32 = 0x10001;
const FORMAT_AUDIO_RATE: u32 = 0x10003;
const FORMAT_AUDIO_CHANNELS: u32 = 0x10004;
const FORMAT_AUDIO_POSITION: u32 = 0x10005;
const MEDIA_TYPE_AUDIO: u32 = 1;
const MEDIA_SUBTYPE_RAW: u32 = 1;
const AUDIO_FORMAT_F32P: u32 = 0x206;
const CHANNEL_FL: u32 = 3;
const CHANNEL_FR: u32 = 4;
const CHANNEL_AUX0: u32 = 0x1000;

/// An object POD (spa/pod/pod.h): `props` as (key, POD type, body words). In u64s, for the
/// POD's 8-byte alignment.
fn object_pod(object_type: u32, id: u32, props: &[(u32, u32, Vec<u32>)]) -> Vec<u64> {
    let mut w = vec![0, TYPE_OBJECT, object_type, id];
    for (key, kind, body) in props {
        w.extend([*key, 0, 4 * body.len() as u32, *kind]);
        w.extend(body);
        if body.len() % 2 == 1 {
            w.push(0); // each POD is padded to 8 bytes
        }
    }
    w[0] = 4 * (w.len() as u32 - 2);
    w.chunks(2).map(|p| p[0] as u64 | (p[1] as u64) << 32).collect()
}

/// An array POD's body: its child POD header (4-byte items of `kind`), then the items.
fn array(kind: u32, items: impl IntoIterator<Item = u32>) -> Vec<u32> {
    [4, kind].into_iter().chain(items).collect()
}

/// The Format param for `id` (EnumFormat or Format): raw float32 planar audio at `rate` with
/// these channel positions, as the session manager wants to see on a sink or source.
fn audio_format_pod(id: u32, rate: u32, positions: &[u32]) -> Vec<u64> {
    object_pod(TYPE_OBJECT_FORMAT, id, &[
        (FORMAT_MEDIA_TYPE, TYPE_ID, vec![MEDIA_TYPE_AUDIO]),
        (FORMAT_MEDIA_SUBTYPE, TYPE_ID, vec![MEDIA_SUBTYPE_RAW]),
        (FORMAT_AUDIO_FORMAT, TYPE_ID, vec![AUDIO_FORMAT_F32P]),
        (FORMAT_AUDIO_RATE, TYPE_INT, vec![rate]),
        (FORMAT_AUDIO_CHANNELS, TYPE_INT, vec![positions.len() as u32]),
        (FORMAT_AUDIO_POSITION, TYPE_ARRAY, array(TYPE_ID, positions.iter().copied())),
    ])
}

/// A port's Latency param (spa/param/latency.h): `quantum` graph cycles plus `frames` at the
/// graph rate, in `direction`. As PipeWire's ALSA nodes report theirs: a sink's input ports in
/// the input direction, a source's output ports in the output direction, one cycle plus what
/// the device adds.
fn latency_pod(direction: u32, quantum: f32, frames: u32) -> Vec<u64> {
    object_pod(TYPE_OBJECT_PARAM_LATENCY, PARAM_LATENCY, &[
        (LATENCY_DIRECTION, TYPE_ID, vec![direction]),
        (LATENCY_MIN_QUANTUM, TYPE_FLOAT, vec![quantum.to_bits()]),
        (LATENCY_MAX_QUANTUM, TYPE_FLOAT, vec![quantum.to_bits()]),
        (LATENCY_MIN_RATE, TYPE_INT, vec![frames]),
        (LATENCY_MAX_RATE, TYPE_INT, vec![frames]),
        (LATENCY_MIN_NS, TYPE_LONG, vec![0, 0]),
        (LATENCY_MAX_NS, TYPE_LONG, vec![0, 0]),
    ])
}

/// The Props param: not muted, every channel at 1.0. Volume and mute set from outside are
/// ignored (levels belong to the unit's own mixer).
fn props_pod(channels: usize) -> Vec<u64> {
    object_pod(TYPE_OBJECT_PROPS, PARAM_PROPS, &[
        (PROP_MUTE, TYPE_BOOL, vec![0]),
        (PROP_CHANNEL_VOLUMES, TYPE_ARRAY, array(TYPE_FLOAT, (0..channels).map(|_| 1.0f32.to_bits()))),
    ])
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub rate: u32,
    pub channels: usize,
    /// Frames per graph cycle.
    pub quantum: u32,
    /// Frames from a cycle's recorded positions to where its output plays.
    pub play_delay: i64,
    /// Frames from a cycle's last recorded position to the cycle's start: an IN URB's worth.
    pub capture_delay: i64,
    /// The channel (0-based) that is FL, the next one FR.
    pub main: usize,
    /// Report every second instead of every 10 s (and at once on trouble).
    pub verbose: bool,
}

pub struct PipeWire {
    pw: &'static Pw,
    thread_loop: Ptr,
    context: Ptr,
    core: Ptr,
    shared: Arc<Shared>,
    opt: Options,
    /// First recorded position of the next cycle.
    next_cycle: Option<i64>,
    graph_position: u64,
    cycles: u64,
    /// Cycles started late (the one before hadn't finished) and cycles skipped.
    late: u64,
    skipped: u64,
    /// Frames played as silence because the graph hadn't delivered them.
    underruns: u64,
    /// Per channel, the highest level sent to the unit and recorded from it since the last
    /// report.
    out_peaks: Vec<f32>,
    in_peaks: Vec<f32>,
    names: Vec<String>,
    ticks: u64,
    /// What the last report said went wrong, to report again as soon as it changes.
    trouble: (u64, u64, u64, u64),
}

impl PipeWire {
    pub fn new(opt: Options) -> Result<PipeWire, String> {
        let pw = Pw::load()?;
        let shared = Arc::new(Shared {
            pw,
            capture: Ring::new(RING_FRAMES, opt.channels),
            playback: Ring::new(RING_FRAMES, opt.channels),
            play_delay: opt.play_delay,
            sink: Node::new(),
            source: Node::new(),
            cycle_pos: AtomicI64::new(0),
            running: AtomicU8::new(0),
            start_pending: AtomicBool::new(false),
            played_to: AtomicI64::new(NOWHERE),
        });
        unsafe {
            (pw.init)(std::ptr::null_mut(), std::ptr::null_mut());
            let thread_loop = (pw.thread_loop_new)(c"ucx-pipewire".as_ptr(), std::ptr::null());
            let context = (pw.context_new)((pw.thread_loop_get_loop)(thread_loop), std::ptr::null_mut(), 0);
            if thread_loop.is_null() || context.is_null() {
                return Err("can't set up a PipeWire context".into());
            }
            (pw.thread_loop_start)(thread_loop);
            (pw.thread_loop_lock)(thread_loop);
            let mut this = PipeWire {
                pw,
                thread_loop,
                context,
                core: std::ptr::null_mut(),
                shared,
                opt,
                next_cycle: None,
                graph_position: 0,
                cycles: 0,
                late: 0,
                skipped: 0,
                underruns: 0,
                out_peaks: vec![0.0; opt.channels],
                in_peaks: vec![0.0; opt.channels],
                names: channel_names(opt.channels, opt.main).into_iter().map(|n| n.0).collect(),
                ticks: 0,
                trouble: (0, 0, 0, 0),
            };
            let made = this.connect();
            (pw.thread_loop_unlock)(thread_loop);
            made?;
            Ok(this)
        }
    }

    /// Makes both filters. With the thread loop locked.
    unsafe fn connect(&mut self) -> Result<(), String> {
        let pw = self.pw;
        self.core = (pw.context_connect)(self.context, std::ptr::null_mut(), 0);
        if self.core.is_null() {
            return Err("can't connect to PipeWire".into());
        }
        let o = self.opt;
        let names = channel_names(o.channels, o.main);
        // the latency each port reports, past one cycle (the graph's clock says "now" at the
        // cycle's start, an IN URB after its last recorded frame): recordings are that IN URB
        // older, and output plays `play_delay` after the cycle's first recorded frame, two
        // cycles and that IN URB past it
        let n = o.quantum as i64;
        let capture = o.capture_delay.max(0) as u32;
        let playback = (o.play_delay - 2 * n - o.capture_delay).max(0) as u32;
        for which in [SINK, SOURCE] {
            let (class, name, prio, direction, prefix, events) = if which == SINK {
                ("Audio/Sink", "fireface_ucx.playback", 30001, DIRECTION_INPUT, "playback", &SINK_EVENTS)
            } else {
                ("Audio/Source", "fireface_ucx.capture", 30000, DIRECTION_OUTPUT, "capture", &SOURCE_EVENTS)
            };
            // a low session priority, so the session manager doesn't pick it as the default
            let props = pw.properties(&format!(
                "media.type=Audio media.class={class} node.name={name} node.description=\"Fireface UCX\" \
                 node.nick=\"Fireface UCX\" node.group=fireface-ucx node.always-process=true \
                 priority.driver={prio} priority.session=100 node.rate=1/{rate} node.latency={q}/{rate} \
                 audio.rate={rate} audio.channels={ch}",
                rate = o.rate,
                q = o.quantum,
                ch = o.channels
            ));
            let cname = CString::new(name).unwrap();
            let filter = (pw.filter_new)(self.core, cname.as_ptr(), props);
            if filter.is_null() {
                return Err(format!("can't make the PipeWire {prefix} node"));
            }
            let node = self.shared.node(which);
            node.filter.store(filter, Ordering::Release);
            let data = Arc::as_ptr(&self.shared) as Ptr;
            (pw.filter_add_listener)(filter, node.hook.get().cast(), events, data);
            let mut ports = Vec::new();
            for (port, channel, _) in &names {
                let props = pw.properties(&format!(
                    "format.dsp=\"32 bit float mono audio\" audio.channel={channel} port.name={prefix}_{port} \
                     port.alias=\"Fireface UCX:{port}\" port.physical=true port.terminal=true"
                ));
                let latency = latency_pod(direction, 1.0, if which == SINK { playback } else { capture });
                let params = [latency.as_ptr() as *const c_void];
                let p = (pw.filter_add_port)(filter, direction, PORT_FLAG_MAP_BUFFERS, 8, props, params.as_ptr().cast(), 1);
                if p.is_null() {
                    return Err(format!("can't add port {prefix}_{port}"));
                }
                ports.push(p as usize);
            }
            let _ = node.ports.set(ports);
            let positions: Vec<u32> = names.iter().map(|n| n.2).collect();
            let pods = [
                audio_format_pod(PARAM_ENUM_FORMAT, o.rate, &positions),
                audio_format_pod(PARAM_FORMAT, o.rate, &positions),
                props_pod(o.channels),
            ];
            let params = pods.each_ref().map(|f| f.as_ptr() as *const c_void);
            let flags = FILTER_FLAG_DRIVER | FILTER_FLAG_RT_PROCESS;
            if (pw.filter_connect)(filter, flags, params.as_ptr().cast(), params.len() as u32) < 0 {
                return Err(format!("can't connect the PipeWire {prefix} node"));
            }
        }
        Ok(())
    }

    /// The node that drives the graph, if it is running: a paused node still says it drives,
    /// but a cycle started on it never ends.
    fn driving(&self) -> u8 {
        for which in [SINK, SOURCE] {
            let node = self.shared.node(which);
            let f = node.filter.load(Ordering::Acquire);
            if !f.is_null() && unsafe { (self.pw.filter_is_driving)(f) } {
                return if node.state.load(Ordering::Acquire) == STATE_STREAMING { which } else { 0 };
            }
        }
        0
    }

    /// Starts as many graph cycles as the recorded frames (up to `to`) allow: one at a time.
    fn run_cycles(&mut self, to: i64) {
        let n = self.opt.quantum as i64;
        let sh = &*self.shared;
        let Some(mut p) = self.next_cycle else { return };
        if to < p + n {
            return;
        }
        if sh.running.load(Ordering::Acquire) != 0 {
            if to - p < 4 * n {
                return; // wait for the running cycle
            }
            // the graph stopped answering: let it go
            sh.running.store(0, Ordering::Release);
            self.late += 1;
        }
        if to - p >= 2 * n {
            let behind = (to - p) / n - 1;
            self.skipped += behind as u64;
            p += behind * n;
            self.graph_position += (behind * n) as u64;
            if self.cycles > 0 {
                self.late += 1;
            }
        }
        let driver = self.driving();
        let clock = if driver == 0 { std::ptr::null_mut() } else { sh.node(driver).position.load(Ordering::Acquire) };
        if clock.is_null() {
            // nothing to drive yet: keep up with the recording
            self.next_cycle = Some(p + n);
            self.graph_position += n as u64;
            return;
        }
        let rate = Fraction { num: 1, denom: self.opt.rate };
        let now = now_ns();
        unsafe {
            let c = &mut *clock;
            c.nsec = now;
            c.rate = rate;
            c.position = self.graph_position;
            c.duration = n as u64;
            c.delay = 0;
            c.rate_diff = 1.0;
            c.next_nsec = now + n as u64 * 1_000_000_000 / self.opt.rate as u64;
            c.target_rate = rate;
            c.target_duration = n as u64;
        }
        sh.cycle_pos.store(p, Ordering::Release);
        sh.start_pending.store(true, Ordering::Release);
        sh.running.store(driver, Ordering::Release);
        unsafe { (self.pw.filter_trigger_process)(sh.node(driver).filter.load(Ordering::Acquire)) };
        self.cycles += 1;
        self.graph_position += n as u64;
        self.next_cycle = Some(p + n);
    }
}

impl Client for PipeWire {
    fn capture(&mut self, position: i64, packet: &InPacket) {
        let ring = &self.shared.capture;
        for c in 0..packet.channels().min(ring.channels) {
            for (f, s) in packet.channel(c).enumerate() {
                unsafe { *ring.slot(position + f as i64, c) = s };
                self.in_peaks[c] = self.in_peaks[c].max(s.abs());
            }
        }
        if self.next_cycle.is_none() {
            self.next_cycle = Some(position);
        }
        self.run_cycles(position + packet.frames() as i64);
    }

    fn playback(&mut self, position: i64, out: &mut [f32]) -> bool {
        let to = self.shared.played_to.load(Ordering::Acquire);
        let ring = &self.shared.playback;
        let channels = ring.channels;
        for (f, frame) in out.chunks_exact_mut(channels).enumerate() {
            let q = position + f as i64;
            if q < to && to - q < (ring.frames / 2) as i64 {
                for (c, s) in frame.iter_mut().enumerate() {
                    *s = unsafe { *ring.slot(q, c) };
                    self.out_peaks[c] = self.out_peaks[c].max(s.abs());
                }
            } else if to != NOWHERE {
                self.underruns += 1;
            }
        }
        true
    }

    fn restarted(&mut self) {
        // positions start over: nothing recorded or played so far lines up any more
        self.next_cycle = None;
        self.shared.played_to.store(NOWHERE, Ordering::Release);
    }

    fn tick(&mut self, stats: &Stats) {
        self.ticks += 1;
        let errors = stats.in_errors + stats.out_errors + stats.resyncs + stats.restarts;
        let trouble = (self.late, self.skipped, self.underruns, errors);
        if !self.opt.verbose && !self.ticks.is_multiple_of(10) && trouble == self.trouble {
            return;
        }
        self.trouble = trouble;
        self.report(stats);
    }
}

impl PipeWire {
    /// Prints the counters and the peak levels since the last report.
    pub fn report(&mut self, stats: &Stats) {
        let driver = match self.driving() {
            SINK => "the sink drives",
            SOURCE => "the source drives",
            _ => "idle",
        };
        eprintln!(
            "{:.2} Hz; graph: {driver}, {} cycles ({} late, {} skipped), {} frames played short; USB: {} IN / {} OUT \
             errors, {} resyncs, {} restarts",
            stats.rate,
            self.cycles,
            self.late,
            self.skipped,
            self.underruns,
            stats.in_errors,
            stats.out_errors,
            stats.resyncs,
            stats.restarts
        );
        eprintln!(
            "  peaks, dBFS: played {}; recorded {}",
            levels(&self.names, &self.out_peaks),
            levels(&self.names, &self.in_peaks)
        );
        self.out_peaks.fill(0.0);
        self.in_peaks.fill(0.0);
    }
}

/// The channels that weren't silent, with their peak in dBFS.
fn levels(names: &[String], peaks: &[f32]) -> String {
    let l: Vec<String> = names
        .iter()
        .zip(peaks)
        .filter(|(_, &p)| p > 0.0)
        .map(|(n, &p)| format!("{n} {:.1}", 20.0 * p.log10()))
        .collect();
    if l.is_empty() {
        "silence".into()
    } else {
        l.join(", ")
    }
}

impl Drop for PipeWire {
    fn drop(&mut self) {
        let pw = self.pw;
        unsafe {
            (pw.thread_loop_lock)(self.thread_loop);
            for which in [SINK, SOURCE] {
                let f = self.shared.node(which).filter.swap(std::ptr::null_mut(), Ordering::AcqRel);
                if !f.is_null() {
                    (pw.filter_destroy)(f);
                }
            }
            if !self.core.is_null() {
                (pw.core_disconnect)(self.core);
            }
            (pw.thread_loop_unlock)(self.thread_loop);
            (pw.thread_loop_stop)(self.thread_loop);
            (pw.context_destroy)(self.context);
            (pw.thread_loop_destroy)(self.thread_loop);
            (pw.deinit)();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_pod_layout() {
        let pod = audio_format_pod(PARAM_ENUM_FORMAT, 48000, &[CHANNEL_FL, CHANNEL_FR, CHANNEL_AUX0 + 2]);
        let w: Vec<u32> = pod.iter().flat_map(|&x| [x as u32, (x >> 32) as u32]).collect();
        // object header and body
        assert_eq!(&w[..4], &[4 * (w.len() as u32 - 2), TYPE_OBJECT, TYPE_OBJECT_FORMAT, PARAM_ENUM_FORMAT]);
        // a prop with an Id: key, flags, then a 4-byte Id POD padded to 8
        assert_eq!(&w[4..10], &[FORMAT_MEDIA_TYPE, 0, 4, TYPE_ID, MEDIA_TYPE_AUDIO, 0]);
        assert_eq!(&w[22..28], &[FORMAT_AUDIO_RATE, 0, 4, TYPE_INT, 48000, 0]);
        // the position array: child POD (4-byte Ids), three of them, padded
        assert_eq!(&w[34..], &[FORMAT_AUDIO_POSITION, 0, 20, TYPE_ARRAY, 4, TYPE_ID, 3, 4, 0x1002, 0]);
    }

    #[test]
    fn latency_pod_layout() {
        let pod = latency_pod(DIRECTION_INPUT, 1.0, 300);
        let w: Vec<u32> = pod.iter().flat_map(|&x| [x as u32, (x >> 32) as u32]).collect();
        assert_eq!(&w[..4], &[4 * (w.len() as u32 - 2), TYPE_OBJECT, TYPE_OBJECT_PARAM_LATENCY, PARAM_LATENCY]);
        assert_eq!(&w[4..10], &[LATENCY_DIRECTION, 0, 4, TYPE_ID, DIRECTION_INPUT, 0]);
        assert_eq!(&w[10..16], &[LATENCY_MIN_QUANTUM, 0, 4, TYPE_FLOAT, 1.0f32.to_bits(), 0]);
        assert_eq!(&w[22..28], &[LATENCY_MIN_RATE, 0, 4, TYPE_INT, 300, 0]);
        // a Long: 8 bytes of body, no padding
        assert_eq!(&w[34..40], &[LATENCY_MIN_NS, 0, 8, TYPE_LONG, 0, 0]);
        assert_eq!(w.len(), 46);
    }
}
