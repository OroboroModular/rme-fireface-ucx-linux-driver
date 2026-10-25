//! Fireface UCX audio on Linux from userspace (`linux/DRIVER_PLAN.md`, option 3).
//!
//!     ucx-stream info
//!     ucx-stream selftest [--rate 44100] [--seconds 10] [--gap-every N] [--packets 8] [--urbs 8]
//!                         [--any-queue]
//!     ucx-stream pipewire [--rate 48000] [--quantum 256] [--play-delay FRAMES] [--main-pair 1]
//!                         [--loopback 3,4] [--seconds S] [--packets 2] [--urbs 24] [--verbose]
//!     ucx-stream meters [--seconds S]
//!     ucx-stream loopback 3,4 | off
//!     ucx-stream mixer-check [--profile]
//!
//! `selftest` streams both ways with loopback on for outputs 3/4 and checks, frame by frame, that
//! an inaudible position code played on playback 3/4 comes back on time (`selftest.rs`).
//! `--gap-every N` leaves every Nth OUT packet empty, to see what the unit plays for positions
//! it gets nothing for. `--any-queue` allows fewer URBs in flight than work, to measure that.
//!
//! `pipewire` puts the unit in the PipeWire graph as the sink and source "Fireface UCX", driven
//! by the unit's clock (`pipewire.rs`), until Ctrl-C or `--seconds`. Stereo streams go to the
//! channel pair `--main-pair` starts at (1-based: 1 = analog 1/2). `--loopback` turns loopback on
//! for the given outputs (1-based) meanwhile, for testing.
//!
//! `meters` shows the unit's level meters (`meters.rs`). It only reads, on the mixer's interface 1,
//! so it can run next to `selftest` or `pipewire`, or the kernel driver.
//!
//! `loopback` turns loopback on for the given outputs (1-based) and off for the others, or off
//! for all, and leaves it so: for testing the kernel driver, which owns interface 0 (vendor
//! requests on endpoint 0 need no interface).
//!
//! `mixer-check` writes the mixer, where nothing can be heard, and measures it on the meters
//! (`mixer_check.rs`): the digital outputs' submixes only, put back at the end. It needs
//! `selftest --seconds 90` running meanwhile.
//!
//! None of the others writes mixer registers. `selftest` and `pipewire` turn loopback off for all
//! outputs at the end, put the starting base rate back and leave interface 0 at alt 0. Needs
//! write access to the unit (`linux/70-fireface-ucx.rules`). Runs its streaming thread at
//! SCHED_FIFO when the user may (e.g. the `audio` group's rtprio limit).

mod engine;
mod meters;
mod mixer_check;
mod pipewire;
mod selftest;

use std::io;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use ucx_protocol::control::{self, request, VendorWrite};
use ucx_protocol::iso;

use engine::{Config, Engine, Format};
use ucx_usb::Device;

static STOP: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_: libc::c_int) {
    STOP.store(true, Ordering::Relaxed);
}

/// Outputs (wIndex of request 21) the driver sets loopback for.
const LOOPBACK_INDICES: u16 = 30;
const RT_PRIORITY: i32 = 70;
const USAGE: &str = "usage: ucx-stream info\n       ucx-stream selftest [--rate HZ] [--seconds S] [--gap-every N] \
                     [--packets N] [--urbs N] [--any-queue]\n       ucx-stream pipewire [--rate HZ] [--quantum N] [--play-delay N] \
                     [--main-pair N] [--loopback 3,4] [--seconds S] [--packets N] [--urbs N] [--verbose]\n       \
                     ucx-stream meters [--seconds S]\n       ucx-stream loopback 3,4 | off\n       \
                     ucx-stream mixer-check [--profile]";

struct Args {
    command: String,
    rate: Option<u32>,
    seconds: Option<f64>,
    gap_every: Option<u64>,
    packets: Option<usize>,
    urbs: Option<usize>,
    quantum: u32,
    play_delay: Option<i64>,
    /// 0-based outputs (request 21's wIndex).
    loopback: Vec<u16>,
    verbose: bool,
    any_queue: bool,
    /// mixer-check: how fast a gain change lands, instead of the checks.
    profile: bool,
    /// 0-based channel that is FL.
    main: usize,
}

fn parse_args() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let mut a = Args {
        command: args.next().unwrap_or_default(),
        rate: None,
        seconds: None,
        gap_every: None,
        packets: None,
        urbs: None,
        quantum: 256,
        play_delay: None,
        loopback: Vec::new(),
        verbose: false,
        any_queue: false,
        profile: false,
        main: 0,
    };
    fn num<T: std::str::FromStr>(flag: &str, v: Option<String>) -> Result<T, String> {
        let v = v.ok_or(format!("{flag} needs a value"))?;
        v.parse().map_err(|_| format!("bad value for {flag}: {v}"))
    }
    fn outputs(flag: &str, v: &str) -> Result<Vec<u16>, String> {
        v.split(',')
            .map(|o| {
                let o: u16 = num(flag, Some(o.to_string()))?;
                if !(1..=LOOPBACK_INDICES).contains(&o) {
                    return Err(format!("{flag} outputs are 1 to {LOOPBACK_INDICES}"));
                }
                Ok(o - 1)
            })
            .collect()
    }
    if a.command == "loopback" {
        let v = args.next().ok_or("loopback needs outputs (3,4) or off")?;
        if v != "off" {
            a.loopback = outputs("loopback", &v)?;
        }
    }
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--rate" => a.rate = Some(num(&flag, args.next())?),
            "--seconds" => a.seconds = Some(num(&flag, args.next())?),
            "--gap-every" => a.gap_every = Some(num(&flag, args.next())?),
            "--packets" => a.packets = Some(num(&flag, args.next())?),
            "--urbs" => a.urbs = Some(num(&flag, args.next())?),
            "--quantum" => a.quantum = num(&flag, args.next())?,
            "--play-delay" => a.play_delay = Some(num(&flag, args.next())?),
            "--verbose" => a.verbose = true,
            "--main-pair" => {
                let m: usize = num(&flag, args.next())?;
                if !(1..=17).contains(&m) {
                    return Err("--main-pair is 1 to 17".into());
                }
                a.main = m - 1;
            }
            "--any-queue" => a.any_queue = true,
            "--profile" => a.profile = true,
            "--loopback" => {
                let v: String = num(&flag, args.next())?;
                a.loopback = outputs(&flag, &v)?;
            }
            _ => return Err(format!("unknown option {flag}")),
        }
    }
    if !["info", "selftest", "pipewire", "meters", "loopback", "mixer-check"].contains(&a.command.as_str()) {
        return Err(format!("unknown command {:?}", a.command));
    }
    if !a.packets.is_none_or(|p| (1..=ucx_usb::MAX_PACKETS).contains(&p)) || a.urbs.is_some_and(|u| u < 2) {
        return Err("--packets 1 to 128, --urbs at least 2".into());
    }
    if !(16..=8192).contains(&a.quantum) {
        return Err("--quantum 16 to 8192".into());
    }
    Ok(a)
}

fn status(dev: &Device) -> io::Result<(u32, Option<u32>)> {
    let word = dev.read(request::STATUS)?;
    Ok((word, control::rate_from_status(word)))
}

fn set_loopback(dev: &Device, outputs: &[u16]) -> io::Result<()> {
    for i in 0..LOOPBACK_INDICES {
        dev.write(VendorWrite { request: request::LOOPBACK, value: outputs.contains(&i) as u16, index: i })?;
    }
    Ok(())
}

fn realtime() {
    let param = libc::sched_param { sched_priority: RT_PRIORITY };
    if unsafe { libc::sched_setscheduler(0, libc::SCHED_FIFO, &param) } != 0 {
        eprintln!("not real-time ({}): running at normal priority", io::Error::last_os_error());
    }
}

/// The command's defaults for packets per URB and URBs, unless given.
fn config(a: &Args, rate: u32, packets: usize, urbs: usize) -> io::Result<Config> {
    let format = Format::new(rate).ok_or_else(|| io::Error::other(format!("unsupported rate {rate}")))?;
    let packets = a.packets.unwrap_or(packets);
    let urbs = a.urbs.unwrap_or(urbs);
    let cfg = Config { format, packets, urbs, lead: iso::OUT_LEAD as i64 };
    if cfg.packets * cfg.urbs < engine::MIN_QUEUE_UFRAMES && !a.any_queue {
        return Err(io::Error::other(format!(
            "--packets × --urbs has to be at least {}: shorter queues lose packets",
            engine::MIN_QUEUE_UFRAMES
        )));
    }
    eprintln!(
        "{rate} Hz, {} channels, {} packets per URB, {} URBs per direction, OUT lead {} frames",
        format.channels, cfg.packets, cfg.urbs, cfg.lead
    );
    Ok(cfg)
}

/// Sets STOP after `seconds`.
fn stop_after(seconds: f64) {
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs_f64(seconds));
        STOP.store(true, Ordering::Relaxed);
    });
}

fn selftest(dev: &mut Device, a: &Args) -> io::Result<bool> {
    let cfg = config(a, a.rate.unwrap_or(44100), 8, 8)?;
    let mut client = selftest::Selftest::new(cfg.format.channels, a.gap_every);
    let mut engine = Engine::new(cfg);
    dev.claim(engine::AUDIO_IF)?;
    let looped: Vec<u16> = selftest::LOOPED.iter().map(|&c| c as u16).collect();
    set_loopback(dev, &looped)?;
    stop_after(a.seconds.unwrap_or(10.0));
    realtime();
    let result = engine.run(dev, &mut client, &STOP);
    let off = set_loopback(dev, &[]);
    result?;
    off?;

    let s = &engine.stats;
    eprintln!(
        "done: {} IN packets, {} IN errors, {} OUT errors, {} resyncs, {} restarts",
        s.in_packets, s.in_errors, s.out_errors, s.resyncs, s.restarts
    );
    eprintln!("loopback delay {:?} frames; played: {}", client.delay, client.played_frames);
    if a.gap_every.is_some() {
        eprintln!("left out: {}", client.gap_frames);
    }
    for e in client.examples() {
        eprintln!("  {e}");
    }
    let clean = client.played_frames.on_time > 0
        && client.played_frames.errors() == 0
        && s.in_errors <= 2 // the first packets after SET_INTERFACE can be empty
        && s.out_errors == 0
        && s.resyncs == 0
        && s.restarts == 0;
    Ok(clean)
}

fn pipewire(dev: &mut Device, a: &Args) -> io::Result<bool> {
    let rate = a.rate.unwrap_or(48000);
    let cfg = config(a, rate, 2, 24)?;
    let fpu = rate as i64 / 8000;
    let n = a.quantum as i64;
    // a cycle starts once its last frame is in (one IN URB after it was recorded), may take up
    // to a cycle to run, and its output has to be there when the OUT URB it goes in is built
    let in_urb = cfg.packets as i64 * fpu;
    let out_queue = (cfg.urbs * cfg.packets) as i64 * fpu;
    let play_delay = a.play_delay.unwrap_or(2 * n + in_urb + cfg.lead + out_queue + rate as i64 / 1000);
    eprintln!(
        "graph cycles of {n} frames ({:.1} ms); output plays {play_delay} frames ({:.1} ms) after the cycle's input",
        n as f64 * 1000.0 / rate as f64,
        play_delay as f64 * 1000.0 / rate as f64
    );
    if a.main + 1 >= cfg.format.channels {
        return Err(io::Error::other(format!("--main-pair: {} channels at {rate} Hz", cfg.format.channels)));
    }
    let opt = pipewire::Options {
        rate,
        channels: cfg.format.channels,
        quantum: a.quantum,
        play_delay,
        capture_delay: in_urb,
        main: a.main,
        verbose: a.verbose,
    };
    let mut engine = Engine::new(cfg);
    dev.claim(engine::AUDIO_IF)?;
    set_loopback(dev, &a.loopback)?;
    let mut client = pipewire::PipeWire::new(opt).map_err(io::Error::other)?;
    if let Some(s) = a.seconds {
        stop_after(s);
    }
    realtime();
    let result = engine.run(dev, &mut client, &STOP);
    client.report(&engine.stats);
    drop(client);
    let off = set_loopback(dev, &[]);
    result?;
    off?;
    Ok(true)
}

fn rate_name(rate: Option<u32>) -> String {
    rate.map_or("?".into(), |r| format!("{r} Hz"))
}

fn run(a: &Args) -> io::Result<bool> {
    let mut dev = Device::open()?;
    let (word, start_rate) = status(&dev)?;
    let firmware = dev.read(request::FIRMWARE)?.swap_bytes();
    eprintln!("Fireface UCX at {}: firmware {firmware:08x}, status {word:08x} ({})", dev.node.display(), rate_name(start_rate));
    let result = match a.command.as_str() {
        "selftest" => selftest(&mut dev, a),
        "pipewire" => pipewire(&mut dev, a),
        "meters" => return meters::run(&mut dev, a.seconds, &STOP),
        "mixer-check" if a.profile => return mixer_check::profile(&mut dev),
        "mixer-check" => return mixer_check::run(&mut dev),
        "loopback" => {
            set_loopback(&dev, &a.loopback)?;
            let on: Vec<u16> = a.loopback.iter().map(|o| o + 1).collect();
            if on.is_empty() {
                eprintln!("loopback off for all outputs");
            } else {
                eprintln!("loopback on for outputs {on:?}, off for the others");
            }
            return Ok(true);
        }
        _ => return Ok(true),
    };
    if let Some((base, _)) = start_rate.and_then(control::split_rate) {
        dev.write(control::base_rate(base).request())?;
    }
    let (word, rate) = status(&dev)?;
    eprintln!("interface 0 at alt 0, status {word:08x} ({})", rate_name(rate));
    result
}

fn main() -> ExitCode {
    let a = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    unsafe {
        libc::signal(libc::SIGINT, on_signal as *const () as libc::sighandler_t);
        libc::signal(libc::SIGTERM, on_signal as *const () as libc::sighandler_t);
    }
    match run(&a) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
