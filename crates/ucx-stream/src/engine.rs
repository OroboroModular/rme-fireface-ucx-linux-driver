//! Streaming on interface 0, both directions, with OUT locked to the unit's frame clock.
//!
//! IN runs first. Its headers give the unit's position per microframe ([`Clock`]). OUT starts
//! 50 ms later, silent, until the reap times of its first URBs line its packets up with IN's
//! ([`Offset`]): OUT packet k goes out in the same microframe as IN packet k + s. From then on
//! each OUT packet's first lead word is the unit's predicted position in its microframe plus
//! [`Config::lead`], and its frame count is whatever keeps the next packet on target, so the lead
//! words run on without gaps and follow the unit's frame counts (`re/NOTES.md`, "OUT timing").

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

use ucx_protocol::control::{self, BaseRate};
use ucx_protocol::iso::{self, InHeader, InPacket};

use ucx_usb::{now_ns, Device, IsoUrb};

pub const AUDIO_IF: u32 = 0;
pub const EP_OUT: u8 = 0x0e;
pub const EP_IN: u8 = 0x8f;

const UFRAME_NS: u64 = 125_000;
/// IN runs alone this long before OUT starts.
const OUT_AFTER_NS: u64 = 50_000_000;
/// OUT runs silent this many microframes before the offset is taken.
const CALIBRATION_UFRAMES: u64 = 1600;
/// The offset is taken again over each stretch of this many microframes, as a check.
const CHECK_UFRAMES: u64 = 800;
/// URBs in flight have to cover at least this many microframes per direction. With 16 or fewer
/// (2 × 8, 4 × 4) the IN stream loses about one packet in six and OUT goes off target, with 24
/// (3 × 8, 2 × 12) it lost its queue once in ten runs, with 32 or more it is clean
/// (`re/NOTES.md`, "Streaming both ways from userspace"). Presumably the xHCI driver's
/// scheduling threshold: a URB queued less than about two frames ahead gets no explicit frame
/// number (`xhci_get_isoc_frame_id`).
pub const MIN_QUEUE_UFRAMES: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct Format {
    pub rate: u32,
    pub base: BaseRate,
    pub speed: u8,
    pub channels: usize,
}

impl Format {
    pub fn new(rate: u32) -> Option<Format> {
        let (base, speed) = control::split_rate(rate)?;
        Some(Format { rate, base, speed, channels: iso::channels(speed) })
    }

    fn frames_per_uframe(&self) -> f64 {
        self.rate as f64 / 8000.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub format: Format,
    /// Packets (microframes) per URB.
    pub packets: usize,
    /// URBs in flight per direction.
    pub urbs: usize,
    /// Where an OUT packet's first frame goes against the unit's position in the packet's
    /// microframe ([`iso::OUT_LEAD`]).
    pub lead: i64,
}

/// What the stream carries. Positions are the unit's frame clock (the IN header's position,
/// unwrapped), the same for both directions: a frame played at position q comes back through the
/// unit's loopback a few frames after q.
pub trait Client {
    /// An IN packet: its frames were recorded at positions `position` .. + `packet.frames()`.
    fn capture(&mut self, position: i64, packet: &InPacket);

    /// The frames that play at positions `position` .. + `out.len() / channels`, interleaved into
    /// `out` (zeros on entry). Asked in order once OUT is locked; `position` only jumps after a
    /// resync. False leaves the packet empty: the unit gets nothing for those positions.
    fn playback(&mut self, position: i64, out: &mut [f32]) -> bool;

    /// Streaming started over (after losing the offset): positions may jump.
    fn restarted(&mut self) {}

    /// About once a second.
    fn tick(&mut self, _stats: &Stats) {}
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub in_packets: u64,
    /// IN packets with a bad status or no valid header, and the latest status.
    pub in_errors: u64,
    pub in_error_status: i32,
    /// OUT packets with a bad status, and the latest status.
    pub out_errors: u64,
    pub out_error_status: i32,
    /// OUT packet k goes out with IN packet k + offset; None while OUT is starting.
    pub offset: Option<i64>,
    /// The offset as measured since the last tick, lowest and highest.
    pub offset_check: Option<(f64, f64)>,
    /// Frames per second, from the IN positions against the microframe count.
    pub rate: f64,
    /// The unit's position after the latest IN packet.
    pub position: i64,
    /// IN positions that didn't follow on from the packet before ([`Clock`]).
    pub position_jumps: u64,
    /// OUT lead words put back on target (lead word continuity broken).
    pub resyncs: u64,
    /// Streaming started over after losing the offset.
    pub restarts: u64,
    /// The longest time spent handling one wakeup since the last tick, in µs.
    pub busiest_us: u64,
}

/// The unit's frame clock, from the IN headers.
struct Clock {
    /// Frames per microframe.
    fpu: f64,
    /// (IN packet index, unit position just after its frames) of the newest IN packet.
    last: Option<(u64, i64)>,
    /// (index, position of its first frame) of the IN packet the rate is measured from: the
    /// first with frames after the latest jump.
    anchor: Option<(u64, i64)>,
    /// Positions that didn't follow on from the packet before. The first packets after
    /// SET_INTERFACE can be stale: frames at position 0, then a jump to where the unit's
    /// counter really is (it runs from the alt setting on, before IN is read).
    jumps: u64,
}

impl Clock {
    fn new(format: &Format) -> Clock {
        Clock { fpu: format.frames_per_uframe(), last: None, anchor: None, jumps: 0 }
    }

    /// Unwraps an IN packet's position: the position of its first frame.
    fn take(&mut self, k: u64, h: InHeader) -> i64 {
        let wrap = iso::POSITION_WRAP as i64;
        let first = match self.last {
            None => h.position as i64,
            Some((lk, la)) => {
                let expect = la + ((k - lk - 1) as f64 * self.fpu).round() as i64;
                let delta = (h.position as i64 - expect).rem_euclid(wrap);
                let delta = if delta >= wrap / 2 { delta - wrap } else { delta };
                // after missed packets, the frame counts' jitter
                if delta.abs() > if k == lk + 1 { 0 } else { 2 } {
                    self.jumps += 1;
                    self.anchor = None;
                }
                expect + delta
            }
        };
        let after = first + h.frames as i64;
        self.last = Some((k, after));
        match self.anchor {
            None if h.frames > 0 => self.anchor = Some((k, first)),
            Some((ak, af)) if k - ak >= 400 => self.fpu = (after - af) as f64 / (k + 1 - ak) as f64,
            _ => {}
        }
        first
    }

    /// The unit's position just after IN packet k's frames.
    fn predict(&self, k: i64) -> i64 {
        let (lk, la) = self.last.expect("an IN packet before OUT");
        la + ((k - lk as i64) as f64 * self.fpu).round() as i64
    }
}

/// Lines the two streams up by reap times. A URB is reaped some time after its last microframe,
/// never before, so the earliest reap against each stream's own packet count says where the
/// stream started, to a fraction of a microframe.
#[derive(Default)]
struct Offset {
    e_in: Option<i64>,
    e_out: Option<i64>,
}

impl Offset {
    /// A URB of either stream whose packets end at index `end` was reaped at `t` (ns).
    fn add(&mut self, out: bool, end: u64, t: u64) {
        let e = t as i64 - (end * UFRAME_NS) as i64;
        let m = if out { &mut self.e_out } else { &mut self.e_in };
        *m = Some(m.map_or(e, |x| x.min(e)));
    }

    /// s in microframes: OUT packet k goes out with IN packet k + s.
    fn get(&self) -> Option<f64> {
        Some((self.e_out? - self.e_in?) as f64 / UFRAME_NS as f64)
    }
}

/// Why a stream session ended.
enum End {
    Stopped,
    /// A queue ran dry or the offset moved: the streams have to start over.
    Lost(String),
}

/// One go at streaming: IN from packet 0, then OUT.
struct Session {
    clock: Clock,
    next_in: u64,
    next_out: u64,
    started: u64,
    /// (end index, reap time) of the newest IN URB.
    last_in_reap: Option<(u64, u64)>,
    out_started: bool,
    offset: Offset,
    s: Option<i64>,
    s_guess: i64,
    /// The OUT packet index where the current check stretch began.
    check_from: u64,
    /// Lead word of the next OUT packet.
    lead: Option<i64>,
}

impl Session {
    fn new(format: &Format) -> Session {
        Session {
            clock: Clock::new(format),
            next_in: 0,
            next_out: 0,
            started: now_ns(),
            last_in_reap: None,
            out_started: false,
            offset: Offset::default(),
            s: None,
            s_guess: 0,
            check_from: 0,
            lead: None,
        }
    }
}

pub struct Engine {
    cfg: Config,
    urbs: Vec<IsoUrb>,
    scratch: Vec<f32>,
    max_frames: i64,
    pub stats: Stats,
}

impl Engine {
    pub fn new(cfg: Config) -> Engine {
        let (out_max, in_max) = iso::max_packet(cfg.format.speed);
        let urbs = (0..cfg.urbs)
            .map(|t| IsoUrb::new(EP_IN, cfg.packets, in_max, t))
            .chain((0..cfg.urbs).map(|t| IsoUrb::new(EP_OUT, cfg.packets, out_max, cfg.urbs + t)))
            .collect();
        let max_frames = iso::max_out_frames(cfg.format.speed);
        Engine {
            cfg,
            urbs,
            scratch: vec![0.0; max_frames * cfg.format.channels],
            max_frames: max_frames as i64,
            stats: Stats::default(),
        }
    }

    /// Streams until `stop` is set. Selects the alt setting and base rate and unmutes playback
    /// first; mutes it and goes back to alt 0 after. (Ring slots the unit gets no frames for
    /// play silence, so stopping OUT anywhere is clean.)
    pub fn run(&mut self, dev: &Device, client: &mut dyn Client, stop: &AtomicBool) -> io::Result<()> {
        let f = self.cfg.format;
        dev.set_alt(AUDIO_IF, control::alt_setting(f.speed) as u32)?;
        dev.write(control::base_rate(f.base).request())?;
        for w in control::stream_start(f.channels as u8) {
            dev.write(w)?;
        }
        let result = loop {
            let mut session = Session::new(&f);
            match self.session(dev, &mut session, client, stop) {
                Ok(End::Stopped) => break Ok(()),
                Ok(End::Lost(why)) => {
                    eprintln!("lost the stream offset ({why}): starting over");
                    self.stats.restarts += 1;
                    self.stats.offset = None;
                    if !self.cancel(dev) {
                        break Err(io::Error::other("URBs still pending after discarding them"));
                    }
                    client.restarted();
                }
                Err(e) => break Err(e),
            }
        };
        for w in control::stream_stop(f.channels as u8) {
            let _ = dev.write(w);
        }
        if !self.cancel(dev) {
            eprintln!("URBs still pending after discarding them");
        }
        let _ = dev.set_alt(AUDIO_IF, 0);
        result
    }

    /// Discards every pending URB and reaps them. False if some never came back: those stay
    /// allocated (the kernel may still write into them when they're reaped), and the engine has
    /// no URBs left.
    fn cancel(&mut self, dev: &Device) -> bool {
        for u in &mut self.urbs {
            dev.discard(u);
        }
        let deadline = now_ns() + 1_000_000_000;
        while self.urbs.iter().any(|u| u.pending) && now_ns() < deadline {
            let _ = dev.wait(10);
            while let Ok(Some(tag)) = dev.reap() {
                self.urbs[tag].pending = false;
            }
        }
        if self.urbs.iter().any(|u| u.pending) {
            std::mem::forget(std::mem::take(&mut self.urbs));
            return false;
        }
        true
    }

    fn session(&mut self, dev: &Device, ss: &mut Session, client: &mut dyn Client, stop: &AtomicBool) -> io::Result<End> {
        let p = self.cfg.packets as u64;
        let n = self.cfg.urbs;
        for u in &mut self.urbs[..n] {
            u.expect();
            u.base = ss.next_in;
            ss.next_in += p;
            dev.submit(u)?;
        }
        let mut last_tick = now_ns();
        let mut check: Option<(f64, f64)> = None;
        loop {
            if stop.load(Ordering::Relaxed) {
                return Ok(End::Stopped);
            }
            dev.wait(20)?;
            let woke = now_ns();
            while let Some(tag) = dev.reap()? {
                let t = now_ns();
                self.urbs[tag].pending = false;
                self.urbs[tag].reaped_at = t;
                if tag < n {
                    self.take_in(tag, ss, client);
                    if self.urbs[..n].iter().all(|u| !u.pending) {
                        return Ok(End::Lost("the IN queue ran dry".into()));
                    }
                    let u = &mut self.urbs[tag];
                    u.expect();
                    u.base = ss.next_in;
                    ss.next_in += p;
                    dev.submit(u)?;
                } else {
                    self.take_out(tag, ss);
                    if self.urbs[n..].iter().all(|u| !u.pending) {
                        return Ok(End::Lost("the OUT queue ran dry".into()));
                    }
                    self.build_out(tag, ss, client);
                    dev.submit(&mut self.urbs[tag])?;
                }
            }

            if !ss.out_started && woke - ss.started > OUT_AFTER_NS && ss.clock.last.is_some() {
                // until the offset is known, OUT is silent and its lead words go by a guess
                let (end, t) = ss.last_in_reap.unwrap();
                let now = end + (now_ns() - t) / UFRAME_NS;
                ss.s_guess = now as i64 + 16;
                ss.offset = Offset::default();
                for tag in n..2 * n {
                    self.build_out(tag, ss, client);
                    dev.submit(&mut self.urbs[tag])?;
                }
                ss.out_started = true;
            }
            let calibrated = ss.out_started && ss.next_out >= CALIBRATION_UFRAMES + n as u64 * p;
            if let (None, true, Some(s)) = (ss.s, calibrated, ss.offset.get()) {
                ss.s = Some(s.round() as i64);
                ss.lead = None; // back on target with the real offset
                ss.offset = Offset::default();
                ss.check_from = ss.next_out;
                self.stats.offset = ss.s;
                eprintln!("OUT packet k goes out with IN packet k + {s:.3}");
            } else if let (Some(s), Some(c)) = (ss.s, ss.offset.get()) {
                if ss.next_out - ss.check_from >= CHECK_UFRAMES {
                    check = Some(check.map_or((c, c), |(lo, hi)| (lo.min(c), hi.max(c))));
                    ss.offset = Offset::default();
                    ss.check_from = ss.next_out;
                    if (c - s as f64).abs() > 0.5 {
                        return Ok(End::Lost(format!("offset {c:.3} instead of {s}")));
                    }
                }
            }

            let done = now_ns();
            self.stats.busiest_us = self.stats.busiest_us.max((done - woke) / 1000);
            if done - last_tick >= 1_000_000_000 {
                last_tick = done;
                self.stats.rate = ss.clock.fpu * 8000.0;
                self.stats.offset_check = check.take();
                client.tick(&self.stats);
                self.stats.busiest_us = 0;
            }
        }
    }

    fn take_in(&mut self, tag: usize, ss: &mut Session, client: &mut dyn Client) {
        let channels = self.cfg.format.channels;
        let u = &self.urbs[tag];
        let end = u.base + u.packets as u64;
        ss.last_in_reap = Some((end, u.reaped_at));
        ss.offset.add(false, end, u.reaped_at);
        for i in 0..u.packets {
            let status = u.status(i);
            let packet = if status == 0 { InPacket::parse(u.received(i), channels).ok() } else { None };
            let Some(packet) = packet else {
                self.stats.in_errors += 1;
                self.stats.in_error_status = status;
                continue;
            };
            let first = ss.clock.take(u.base + i as u64, packet.header);
            self.stats.in_packets += 1;
            self.stats.position = first + packet.frames() as i64;
            self.stats.position_jumps = ss.clock.jumps;
            client.capture(first, &packet);
        }
    }

    fn take_out(&mut self, tag: usize, ss: &mut Session) {
        let u = &self.urbs[tag];
        let end = u.base + u.packets as u64;
        ss.offset.add(true, end, u.reaped_at);
        for i in 0..u.packets {
            if u.status(i) != 0 {
                self.stats.out_errors += 1;
                self.stats.out_error_status = u.status(i);
            }
        }
    }

    fn build_out(&mut self, tag: usize, ss: &mut Session, client: &mut dyn Client) {
        let channels = self.cfg.format.channels;
        let (lead, max_frames) = (self.cfg.lead, self.max_frames);
        let s = ss.s.unwrap_or(ss.s_guess);
        let live = ss.s.is_some();
        let base = ss.next_out;
        ss.next_out += self.cfg.packets as u64;
        let (scratch, stats) = (&mut self.scratch, &mut self.stats);
        let u = &mut self.urbs[tag];
        u.base = base;
        u.fill(|i, room| {
            let k = (base + i as u64) as i64;
            let target = ss.clock.predict(k + s) + lead;
            let l = match ss.lead {
                Some(l) if (l - target).abs() <= max_frames => l,
                Some(_) if live => {
                    stats.resyncs += 1;
                    target
                }
                _ => target,
            };
            let n = (ss.clock.predict(k + 1 + s) + lead - l).clamp(0, max_frames);
            ss.lead = Some(l + n);
            let frames = &mut scratch[..n as usize * channels];
            frames.fill(0.0);
            if live && !client.playback(l, frames) {
                return 0;
            }
            let position = l.rem_euclid(iso::POSITION_WRAP as i64) as u16;
            iso::write_out(room, position, channels, n as usize, |c, f| frames[f * channels + c])
        });
    }
}
