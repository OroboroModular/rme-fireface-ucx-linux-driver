//! `ucx-stream meters`: the unit's level meters, read from bulk IN `0x85` on interface 1. Only
//! reads: the unit answers each read at once with the next kind's packet, nothing turns them on
//! (`re/NOTES.md`, "Level meters"), so reads are paced like TotalMix's, one every
//! [`READ_EVERY`]. Interface 1 is the mixer's, so this runs next to a stream.

use std::io::{self, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ucx_protocol::meters::{peak_dbfs, power_dbfs, MeterKind, MeterPacket, PACKET_LEN};
use ucx_protocol::regs::{CHANNELS, INPUT_NAMES, OUTPUT_NAMES};

use ucx_usb::Device;

pub const MIXER_IF: u32 = 1;
const EP_METERS: u8 = 0x85;
/// About 240 packets a second, 48 of each kind, as TotalMix reads them.
const READ_EVERY: Duration = Duration::from_micros(4167);
const KINDS: [MeterKind; 5] = [
    MeterKind::InputsA,
    MeterKind::InputsB,
    MeterKind::Playback,
    MeterKind::OutputsPreFader,
    MeterKind::OutputsPostFader,
];

/// The loudest reading of each kind over one display period.
#[derive(Default)]
struct Window {
    packets: [u32; KINDS.len()],
    power: [[u64; CHANNELS]; KINDS.len()],
    peak: [[u32; CHANNELS]; KINDS.len()],
    bad: u32,
    timeouts: u32,
}

impl Window {
    fn add(&mut self, p: &MeterPacket) {
        let k = KINDS.iter().position(|&kind| kind == p.kind).unwrap();
        self.packets[k] += 1;
        for c in 0..CHANNELS {
            self.power[k][c] = self.power[k][c].max(p.power[c]);
            self.peak[k][c] = self.peak[k][c].max(p.peak[c]);
        }
    }

    fn show(&self, seconds: f64, out: &mut impl Write) -> io::Result<()> {
        let db = |x: f32| if x.is_finite() { format!("{x:6.1}") } else { "  -inf".into() };
        let level = |k: usize, c: usize| format!("{} {}", db(peak_dbfs(self.peak[k][c])), db(power_dbfs(self.power[k][c])));
        let index = |kind| KINDS.iter().position(|&k| k == kind).unwrap();
        let (inputs, playback, post) =
            (index(MeterKind::InputsA), index(MeterKind::Playback), index(MeterKind::OutputsPostFader));
        writeln!(
            out,
            "{seconds:.1} s: {:?} packets (inputs A/B, playback, outputs pre/post), {} bad, {} timeouts",
            self.packets, self.bad, self.timeouts
        )?;
        writeln!(out, "{:<8} {:>6} {:>6}    {:<8} {:>6} {:>6}  {:>6} {:>6}", "input", "peak", "rms", "output", "play", "rms", "out", "rms")?;
        for c in 0..CHANNELS {
            writeln!(
                out,
                "{:<8} {}    {:<8} {}  {}",
                INPUT_NAMES[c],
                level(inputs, c),
                OUTPUT_NAMES[c],
                level(playback, c),
                level(post, c)
            )?;
        }
        Ok(())
    }
}

/// Shows the meters until `stop` or `seconds`: in place on a terminal, else a table a second.
/// Levels are the loudest of the period in dBFS: peak, and RMS over the unit's meter window.
pub fn run(dev: &mut Device, seconds: Option<f64>, stop: &AtomicBool) -> io::Result<bool> {
    dev.claim(MIXER_IF)?;
    let live = io::stdout().is_terminal();
    let period = Duration::from_millis(if live { 200 } else { 1000 });
    let start = Instant::now();
    let mut next = start + period;
    let mut window = Window::default();
    let mut read_at = start;
    let mut total = 0u64;
    let mut buf = [0u8; 512];
    while !stop.load(Ordering::Relaxed) && seconds.is_none_or(|s| start.elapsed().as_secs_f64() < s) {
        read_at += READ_EVERY;
        let now = Instant::now();
        if read_at > now {
            std::thread::sleep(read_at - now);
        } else {
            read_at = now;
        }
        match dev.bulk(EP_METERS, &mut buf, 250) {
            Ok(n) => match MeterPacket::parse(&buf[..n]) {
                Ok(p) => {
                    window.add(&p);
                    total += 1;
                }
                Err(e) => {
                    if window.bad == 0 {
                        eprintln!("not a meter packet ({e:?}, {n} bytes, expected {PACKET_LEN})");
                    }
                    window.bad += 1;
                }
            },
            Err(e) if e.kind() == io::ErrorKind::TimedOut => window.timeouts += 1,
            Err(e) => return Err(e),
        }
        if Instant::now() >= next {
            let mut out = io::stdout().lock();
            if live {
                write!(out, "\x1b[H\x1b[J")?;
            }
            window.show(start.elapsed().as_secs_f64(), &mut out)?;
            out.flush()?;
            window = Window::default();
            next += period;
        }
    }
    let rate = total as f64 / start.elapsed().as_secs_f64();
    eprintln!("{total} meter packets, {rate:.0} per second");
    Ok(total > 0)
}
