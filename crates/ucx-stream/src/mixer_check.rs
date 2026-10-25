//! `ucx-stream mixer-check`: writes the mixer from Linux (bulk OUT `0x0c` on interface 1) and
//! measures what the unit does with it on its level meters, where nothing can be heard.
//!
//! Only the submixes of the digital outputs (SPDIF and ADAT, nothing plugged into them), SPDIF's
//! own processing (dynamics, EQ, low cut) and the ADAT 1/2 output fader are written: every word is checked against that before anything goes
//! out (`ucx_mixer::digital_only`), and they're put back as found at the end (the submixes as TotalMix left
//! them, the fader at the level measured before). The words come from `ucx_mixer`'s model.
//!
//! The signals: the selftest's code on playback 3/4 (`ucx-stream selftest --seconds 90` in
//! another process meanwhile: 128 … 255 LSB, below −90 dBFS) and the unit's own input noise on
//! Mic 1. Gains are measured as the ratio of an output's mean power to its source's, from meter
//! packets of both read over the same second; the expected gain is the one the matrix word
//! written encodes.
//!
//! The unit sends status words on bulk IN `0x8d` all the time (about 100 a second, `0x3f..`),
//! which TotalMix reads all the time; unread, they fill a queue of about 450 words, and then a
//! long run of writes stalls (a 720-word restore timed out on `0x0c`). So a thread reads `0x8d`
//! throughout and tallies what comes, and writes go in bursts of at most 63 words, 20 ms apart,
//! as TotalMix's do.
//!
//! The meters' power falls slowly after a drop (`--profile` measures it), so each measurement
//! waits [`SETTLE`] first.

use std::collections::BTreeMap;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use ucx_mixer::{digital_only, Image, Mixer, Send, Settings, Source, OUTPUTS};
use ucx_protocol::meters::{MeterKind, MeterPacket, POWER_FLOOR};
use ucx_protocol::regs::{matrix_reg, CHANNELS};
use ucx_protocol::values::MatrixGain;
use ucx_protocol::word::{encode_burst, Word};
use ucx_usb::Device;

pub const MIXER_IF: u32 = 1;
const EP_WORDS: u8 = 0x0c;
const EP_REPLIES: u8 = 0x8d;
const EP_METERS: u8 = 0x85;
/// Between two bursts of words.
const BURST_GAP: Duration = Duration::from_millis(20);
const READ_EVERY: Duration = Duration::from_micros(4167);
/// After a write, before measuring: the meters' power has ballistics, falling about 24 dB a
/// second (linear in dB) and rising to within 0.1 dB in about 0.7 s (`--profile`), so a drop
/// to silence from 0 dB takes about 2.5 s to show.
const SETTLE: Duration = Duration::from_secs(3);

/// Output strips written: SPDIF, ADAT 1/2 (their submixes), and ADAT 1/2's output level.
const SPDIF: usize = 4;
const ADAT12: usize = 5;
/// The playback strip carrying the code (playback 3/4), and Mic 1.
const CODE: Source = Source::Playback(1);
const MIC1: Source = Source::Input(0);
/// Channels: the code on playback 3 and 4, Mic 1, SPDIF L/R, ADAT 1/2.
const PB3: usize = 2;
const PB4: usize = 3;
const IN1: usize = 0;
const SPDIF_L: usize = 8;
const SPDIF_R: usize = 9;
const ADAT1: usize = 10;

/// Mean power per channel over a measurement, as a fraction of full scale (1.0 = 0 dBFS RMS).
struct Power {
    inputs: [f64; CHANNELS],
    playback: [f64; CHANNELS],
    pre: [f64; CHANNELS],
    post: [f64; CHANNELS],
}

fn db(p: f64) -> f64 {
    10.0 * p.log10()
}

/// Below this many dB under its source an output counts as silent.
const SILENT_DB: f64 = 40.0;

struct Unit<'a> {
    dev: &'a Device,
    sent: usize,
}

impl Unit<'_> {
    /// Sends words, after checking every one of them against [`digital_only`].
    fn send(&mut self, words: &[Word]) -> io::Result<()> {
        if let Some(w) = words.iter().find(|w| !digital_only(**w)) {
            return Err(io::Error::other(format!("refusing to write {:04x}:{:04x}: not a digital output's", w.addr, w.value)));
        }
        for chunk in words.chunks(63) {
            let mut bytes = encode_burst(chunk);
            self.dev.bulk(EP_WORDS, &mut bytes, 1000)?;
            std::thread::sleep(BURST_GAP);
        }
        self.sent += words.len();
        Ok(())
    }

    /// Waits for the change to settle, then averages the meters over `seconds`.
    fn measure(&self, seconds: f64) -> io::Result<Power> {
        std::thread::sleep(SETTLE);
        let mut sums = [[0.0f64; CHANNELS]; 4];
        let mut counts = [0u32; 4];
        let end = Instant::now() + Duration::from_secs_f64(seconds);
        let mut read_at = Instant::now();
        let mut buf = [0u8; 512];
        while Instant::now() < end {
            read_at += READ_EVERY;
            let now = Instant::now();
            if read_at > now {
                std::thread::sleep(read_at - now);
            }
            let n = match self.dev.bulk(EP_METERS, &mut buf, 250) {
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(e) => return Err(e),
            };
            let Ok(p) = MeterPacket::parse(&buf[..n]) else { continue };
            let k = match p.kind {
                MeterKind::InputsA => 0,
                MeterKind::Playback => 1,
                MeterKind::OutputsPreFader => 2,
                MeterKind::OutputsPostFader => 3,
                MeterKind::InputsB => continue,
            };
            counts[k] += 1;
            for (sum, &power) in sums[k].iter_mut().zip(&p.power) {
                let v = if power <= POWER_FLOOR { 0 } else { power };
                *sum += v as f64 / (1u64 << 55) as f64;
            }
        }
        if counts.contains(&0) {
            return Err(io::Error::other(format!("meter packets missing: {counts:?}")));
        }
        let mean = |k: usize| sums[k].map(|s| s / counts[k] as f64);
        Ok(Power { inputs: mean(0), playback: mean(1), pre: mean(2), post: mean(3) })
    }
}

/// After sending `words`, the output's power against its source's in 100 ms slices for `seconds`
/// (dB, −∞ for silent): how a change lands.
fn timeline(unit: &mut Unit, words: &[Word], out: usize, source: usize, seconds: f64) -> io::Result<Vec<f64>> {
    unit.send(words)?;
    let start = Instant::now();
    let slices = (seconds * 10.0) as usize;
    let mut sums = vec![[0.0f64; 2]; slices];
    let mut counts = vec![[0u32; 2]; slices];
    let mut buf = [0u8; 512];
    let mut read_at = start;
    loop {
        let i = (start.elapsed().as_secs_f64() * 10.0) as usize;
        if i >= slices {
            break;
        }
        read_at += READ_EVERY;
        let now = Instant::now();
        if read_at > now {
            std::thread::sleep(read_at - now);
        }
        let n = match unit.dev.bulk(EP_METERS, &mut buf, 250) {
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(e),
        };
        let Ok(p) = MeterPacket::parse(&buf[..n]) else { continue };
        let (k, c) = match p.kind {
            MeterKind::OutputsPostFader => (0, out),
            MeterKind::Playback => (1, source),
            _ => continue,
        };
        let v = if p.power[c] <= POWER_FLOOR { 0 } else { p.power[c] };
        sums[i][k] += v as f64;
        counts[i][k] += 1;
    }
    Ok((0..slices)
        .map(|i| {
            let mean = |k: usize| sums[i][k] / counts[i][k].max(1) as f64;
            ratio(mean(0), mean(1))
        })
        .collect())
}

/// How fast a gain change lands on the outputs: playback 3 into SPDIF L stepped 0 → −20 dB →
/// off → 0 dB, 3 s each, in 100 ms slices.
pub fn profile(dev: &mut Device) -> io::Result<bool> {
    dev.claim(MIXER_IF)?;
    let dev = &*dev;
    let stop = AtomicBool::new(false);
    let replies = Mutex::new(Replies::default());
    std::thread::scope(|s| {
        s.spawn(|| drain(dev, &stop, &replies));
        let mut unit = Unit { dev, sent: 0 };
        let mut model = Mixer::totalmix();
        let result = (|| {
            unit.send(&model.set_send(SPDIF, CODE, Send { level: 0, pan: 0 }))?;
            std::thread::sleep(Duration::from_secs(3));
            for (level, what) in [(-200, "0 -> -20 dB"), (ucx_mixer::LEVEL_OFF, "-20 dB -> off"), (0, "off -> 0 dB")] {
                let words = model.set_send(SPDIF, CODE, Send { level, pan: 0 });
                let t = timeline(&mut unit, &words, SPDIF_L, PB3, 3.0)?;
                let cells: Vec<String> = t.iter().map(|d| if d.is_finite() { format!("{d:.1}") } else { "-inf".into() }).collect();
                eprintln!("{what}, 100 ms slices: {}", cells.join(" "));
            }
            Ok(())
        })();
        let restored = restore(&mut unit, &mut model, None);
        stop.store(true, Ordering::Relaxed);
        restored.and(result).map(|()| true)
    })
}

/// What came on `0x8d`: per address, how many words and the latest value; and words that
/// didn't have odd parity.
#[derive(Default)]
struct Replies {
    words: u64,
    by_addr: BTreeMap<u16, (u64, u16)>,
    bad: u64,
}

/// Reads `0x8d` until `stop`.
fn drain(dev: &Device, stop: &AtomicBool, replies: &Mutex<Replies>) {
    let mut buf = [0u8; 512];
    while !stop.load(Ordering::Relaxed) {
        let n = match dev.bulk(EP_REPLIES, &mut buf, 50) {
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("reading 0x8d: {e}");
                return;
            }
        };
        let mut r = replies.lock().unwrap_or_else(PoisonError::into_inner);
        for b in buf[..n].chunks_exact(4) {
            r.words += 1;
            match Word::decode(u32::from_le_bytes(b.try_into().unwrap())) {
                Ok(w) => {
                    let e = r.by_addr.entry(w.addr).or_insert((0, 0));
                    *e = (e.0 + 1, w.value);
                }
                Err(_) => r.bad += 1,
            }
        }
    }
}

/// The gain the latest matrix word for `output` ← `source` channel in `words` encodes, in dB.
fn written_db(words: &[Word], output: usize, source: ucx_protocol::regs::Source) -> Option<f64> {
    let reg = matrix_reg(output as u8, source);
    words.iter().rev().find(|w| w.addr == reg).map(|w| MatrixGain(w.value).db())
}

/// What a measured gain should be.
enum Expect {
    /// Within a tolerance of a gain (dB).
    Near(f64, f64),
    Silent,
    /// More or less than a gain (dB): an EQ or a low cut, whose effect on the code isn't worked out.
    Above(f64),
    Below(f64),
}

/// One measured gain against its expectation.
struct Check {
    what: String,
    measured: f64,
    expect: Expect,
}

impl Check {
    /// A gain within `tolerance` of `expected`, or silence for `None`.
    fn near(what: impl Into<String>, measured: f64, expected: Option<f64>, tolerance: f64) -> Check {
        let expect = expected.map_or(Expect::Silent, |e| Expect::Near(e, tolerance));
        Check { what: what.into(), measured, expect }
    }

    fn ok(&self) -> bool {
        match self.expect {
            Expect::Near(e, tolerance) => (self.measured - e).abs() <= tolerance,
            Expect::Silent => self.measured <= -SILENT_DB,
            Expect::Above(e) => self.measured > e,
            Expect::Below(e) => self.measured < e,
        }
    }

    fn line(&self) -> String {
        let expected = match self.expect {
            Expect::Near(e, tolerance) => format!("{e:+.2} ± {tolerance:.2}"),
            Expect::Silent => format!("silent (< -{SILENT_DB:.0})"),
            Expect::Above(e) => format!("above {e:+.1}"),
            Expect::Below(e) => format!("below {e:+.1}"),
        };
        let measured = if self.measured.is_finite() { format!("{:+.2}", self.measured) } else { "-inf".into() };
        format!("{} {:<52} {:>8} dB, expected {expected}", if self.ok() { "ok " } else { "BAD" }, self.what, measured)
    }
}

/// An output's power against its source's, in dB (−∞ when the output is silent).
fn ratio(out: f64, source: f64) -> f64 {
    if out <= 0.0 {
        f64::NEG_INFINITY
    } else {
        db(out / source)
    }
}

pub fn run(dev: &mut Device) -> io::Result<bool> {
    dev.claim(MIXER_IF)?;
    let dev = &*dev;
    let stop = AtomicBool::new(false);
    let replies = Mutex::new(Replies::default());
    let started = Instant::now();
    let (result, restored) = std::thread::scope(|s| {
        s.spawn(|| drain(dev, &stop, &replies));
        let mut unit = Unit { dev, sent: 0 };
        let mut model = Mixer::totalmix();
        let mut found = None;
        let mut checks = Vec::new();
        let result = steps(&mut unit, &mut model, &mut checks, &mut found);
        // back as found, whatever happened: the digital outputs' submixes as TotalMix left them,
        // and ADAT 1/2's level as measured
        let restored = restore(&mut unit, &mut model, found).and_then(|()| {
            let p = unit.measure(1.0)?;
            for (what, c) in [("SPDIF L", SPDIF_L), ("SPDIF R", SPDIF_R), ("ADAT 1", ADAT1)] {
                let measured = ratio(p.post[c], p.playback[PB3]);
                report(&mut checks, Check::near(format!("{what} after putting it back"), measured, None, 0.0));
            }
            Ok(())
        });
        stop.store(true, Ordering::Relaxed);
        let bad = checks.iter().filter(|c| !c.ok()).count();
        eprintln!("{} words written, {} checks, {bad} off", unit.sent, checks.len());
        (result.map(|()| bad == 0), restored)
    });
    let r = replies.lock().unwrap_or_else(PoisonError::into_inner);
    let seconds = started.elapsed().as_secs_f64();
    eprintln!("0x8d: {} words in {seconds:.0} s ({:.0} a second), {} without odd parity; by address (count, last value):", r.words, r.words as f64 / seconds, r.bad);
    for (addr, (count, value)) in &r.by_addr {
        eprintln!("  {addr:04x}: {count:6} words, last {value:04x}");
    }
    restored?;
    result
}

/// Prints a check as it's made and keeps it.
fn report(checks: &mut Vec<Check>, c: Check) {
    eprintln!("{}", c.line());
    checks.push(c);
}

/// The digital outputs' submixes and processing back as TotalMix left them, and ADAT 1/2's level
/// as found.
fn restore(unit: &mut Unit, model: &mut Mixer, found: Option<i16>) -> io::Result<()> {
    let totalmix = Mixer::totalmix();
    let mut words = Vec::new();
    for submix in SPDIF..OUTPUTS.len() {
        for i in 0..ucx_mixer::INPUTS.len() {
            words.extend(model.set_send(submix, Source::Input(i), totalmix.send(submix, Source::Input(i))));
        }
        for p in 0..ucx_mixer::PLAYBACK.len() {
            words.extend(model.set_send(submix, Source::Playback(p), totalmix.send(submix, Source::Playback(p))));
        }
    }
    for o in SPDIF..OUTPUTS.len() {
        words.extend(model.set_output_settings(o, Settings::TOTALMIX));
    }
    if let Some(level) = found {
        words.extend(model.set_output_level(ADAT12, level));
    }
    eprintln!("putting back: {} words", words.len());
    unit.send(&words)
}

fn steps(unit: &mut Unit, model: &mut Mixer, checks: &mut Vec<Check>, found_level: &mut Option<i16>) -> io::Result<()> {
    let pb = |c: usize| ucx_protocol::regs::Source::Playback(c as u8);
    let input = |c: usize| ucx_protocol::regs::Source::Input(c as u8);

    let base = unit.measure(1.0)?;
    eprintln!(
        "before: playback 3 {:.1} dBFS, playback 4 {:.1}, Mic 1 {:.1}; SPDIF L/R and ADAT 1/2 out {:.1} / {:.1} / {:.1}",
        db(base.playback[PB3]),
        db(base.playback[PB4]),
        db(base.inputs[IN1]),
        db(base.post[SPDIF_L]),
        db(base.post[SPDIF_R]),
        db(base.post[ADAT1])
    );
    if db(base.playback[PB3]) < -100.0 {
        return Err(io::Error::other("no code on playback 3: run `ucx-stream selftest --seconds 90` meanwhile"));
    }
    for (what, c) in [("SPDIF L", SPDIF_L), ("SPDIF R", SPDIF_R), ("ADAT 1", ADAT1)] {
        report(checks, Check::near(format!("{what} before any write"), ratio(base.post[c], base.playback[PB3]), None, 0.0));
    }

    // the code into the SPDIF submix at several levels and balances
    for (level, pan) in [(0, 0), (-60, 0), (-61, 0), (-200, 0), (60, 0), (0, 50), (0, -100)] {
        let words = model.set_send(SPDIF, CODE, Send { level, pan });
        unit.send(&words)?;
        let p = unit.measure(1.0)?;
        let (l, r) = (written_db(&words, SPDIF_L, pb(PB3)).unwrap(), written_db(&words, SPDIF_R, pb(PB4)).unwrap());
        let what = |side: &str| format!("playback 3/4 -> SPDIF {side} at {:.1} dB, balance {pan}", level as f64 / 10.0);
        let expect = |g: f64| if g.is_finite() { Some(g) } else { None };
        report(checks, Check::near(what("L"), ratio(p.post[SPDIF_L], p.playback[PB3]), expect(l), 0.3));
        // (playback 4's code moves slowly, so its power wanders within a second)
        report(checks, Check::near(what("R"), ratio(p.post[SPDIF_R], p.playback[PB4]), expect(r), 1.0));
    }
    let words = model.set_send(SPDIF, CODE, Send::OFF);
    unit.send(&words)?;
    let p = unit.measure(1.0)?;
    report(checks, Check::near("playback 3/4 -> SPDIF L off", ratio(p.post[SPDIF_L], p.playback[PB3]), None, 0.0));

    // into ADAT 1/2 instead: SPDIF stays silent; then ADAT 1/2's output fader against its pre-fader meter
    let words = model.set_send(ADAT12, CODE, Send { level: 0, pan: 0 });
    unit.send(&words)?;
    let p = unit.measure(1.0)?;
    report(checks, Check::near("playback 3/4 -> ADAT 1 at 0 dB", ratio(p.post[ADAT1], p.playback[PB3]), Some(0.0), 0.3));
    report(checks, Check::near("SPDIF L meanwhile", ratio(p.post[SPDIF_L], p.playback[PB3]), None, 0.0));
    let found = ratio(p.post[ADAT1], p.pre[ADAT1]);
    eprintln!("ADAT 1/2's output level as found: {found:+.2} dB (post against pre-fader meter)");
    let level = (found * 10.0).round().clamp(-650.0, 60.0) as i16;
    *found_level = Some(level);
    unit.send(&model.set_output_level(ADAT12, -100))?;
    let p = unit.measure(1.0)?;
    report(checks, Check::near("ADAT 1/2 output level -10.0 dB", ratio(p.post[ADAT1], p.pre[ADAT1]), Some(-10.0), 0.3));
    unit.send(&model.set_output_level(ADAT12, level))?;
    let p = unit.measure(1.0)?;
    report(checks, Check::near("ADAT 1/2 output level back", ratio(p.post[ADAT1], p.pre[ADAT1]), Some(found), 0.3));
    unit.send(&model.set_send(ADAT12, CODE, Send::OFF))?;

    // SPDIF's own processing, the code into its submix at 0 dB meanwhile. With both ratios 1:1 the
    // dynamics are its gain alone. An EQ boost and a low cut are only checked for their
    // direction: the code (a 128-sample sawtooth, 128…255 LSB) is about 96 % DC by power, so a
    // boost around its 3rd harmonic lifts it a few tenths of a dB (+12 dB at 1 kHz: about +0.35
    // worked out, +0.33 measured), while a low cut, which takes the DC, takes most of it.
    unit.send(&model.set_send(SPDIF, CODE, Send { level: 0, pan: 0 }))?;
    let mut s = Settings::TOTALMIX;
    let mut step = |unit: &mut Unit, model: &mut Mixer, s: Settings, what: &str, expect: Expect| -> io::Result<()> {
        unit.send(&model.set_output_settings(SPDIF, s))?;
        let p = unit.measure(1.0)?;
        let measured = ratio(p.post[SPDIF_L], p.playback[PB3]);
        report(checks, Check { what: format!("SPDIF out, {what}"), measured, expect });
        Ok(())
    };
    s.dynamics.on = true;
    s.dynamics.gain = -100;
    step(unit, model, s, "dynamics on, 1:1, gain -10 dB", Expect::Near(-10.0, 0.3))?;
    s.dynamics.gain = 100;
    step(unit, model, s, "dynamics on, 1:1, gain +10 dB", Expect::Near(10.0, 0.3))?;
    s.dynamics.on = false;
    step(unit, model, s, "dynamics off (gain +10 dB kept)", Expect::Near(0.0, 0.3))?;
    s.eq.on = true;
    step(unit, model, s, "EQ on, every band at 0 dB", Expect::Near(0.0, 0.3))?;
    s.eq.bands[1].gain = 120;
    step(unit, model, s, "EQ band 2 +12 dB at 1 kHz", Expect::Above(0.1))?;
    s.eq = Settings::TOTALMIX.eq;
    s.low_cut = ucx_mixer::settings::LowCut { on: true, freq: 500, slope: 3 };
    step(unit, model, s, "low cut at 500 Hz, 24 dB/oct", Expect::Below(-10.0))?;
    step(unit, model, Settings::TOTALMIX, "back as found", Expect::Near(0.0, 0.3))?;

    // a stereo strip's width, still into SPDIF at 0 dB: -100 swaps its sides. It's the same in
    // every submix, so only the digital outputs' share of its words goes out (and of putting
    // it back): the analog submixes never see it.
    let digital = |words: Vec<Word>| -> Vec<Word> { words.into_iter().filter(|w| digital_only(*w)).collect() };
    unit.send(&digital(model.set_image(CODE, Image { phase: [false; 2], width: -100 })))?;
    let p = unit.measure(1.0)?;
    report(checks, Check::near("width -100: playback 4 -> SPDIF L", ratio(p.post[SPDIF_L], p.playback[PB4]), Some(0.0), 1.0));
    report(checks, Check::near("width -100: playback 3 -> SPDIF R", ratio(p.post[SPDIF_R], p.playback[PB3]), Some(0.0), 0.3));
    unit.send(&digital(model.set_image(CODE, Image::PLAIN)))?;
    unit.send(&model.set_send(SPDIF, CODE, Send::OFF))?;

    // Mic 1's noise into the SPDIF submix: a mono input, centred and hard left
    for (pan, l, r) in [(0, Some(-3.01), Some(-3.01)), (-100, Some(0.0), None)] {
        let words = model.set_send(SPDIF, MIC1, Send { level: 0, pan });
        unit.send(&words)?;
        let p = unit.measure(2.0)?;
        let (wl, wr) = (written_db(&words, SPDIF_L, input(IN1)), written_db(&words, SPDIF_R, input(IN1)));
        eprintln!("Mic 1 pan {pan}: written {:?} / {:?} dB", wl.map(|g| (g * 100.0).round() / 100.0), wr.map(|g| (g * 100.0).round() / 100.0));
        // (noise: its power wanders more than the code's)
        report(checks, Check::near(format!("Mic 1 -> SPDIF L at 0 dB, pan {pan}"), ratio(p.post[SPDIF_L], p.inputs[IN1]), l, 1.0));
        report(checks, Check::near(format!("Mic 1 -> SPDIF R at 0 dB, pan {pan}"), ratio(p.post[SPDIF_R], p.inputs[IN1]), r, 1.0));
    }

    Ok(())
}
