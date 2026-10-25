//! The link to the unit, on a thread of its own: finds the UCX, claims its mixer interface (1),
//! reads the level meters from bulk IN `0x85`, paced like TotalMix (`ucx-stream meters`), and
//! the status word for the rate, and sends the words the window queues on bulk OUT `0x0c`.
//!
//! What may be sent is the window's choice ([`Reach`]), checked again here word by word: nothing,
//! the digital outputs' words only (`ucx_mixer::digital_only`), or all of them. Words go in
//! bursts of at most 63, 20 ms apart, while another thread reads the status words the unit sends
//! on bulk IN `0x8d` all the time: unread, they fill up and a long run of writes stalls
//! (`re/NOTES.md`, "Writing the mixer from Linux").
//!
//! Interface 1 is free next to the audio drivers (`ucx-stream` and `snd-fireface-ucx` claim
//! interface 0), but only one program can claim it: `ucx-stream meters` running makes it busy.

use std::collections::VecDeque;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use ucx_mixer::digital_only;
use ucx_protocol::control::{rate_from_status, request};
use ucx_protocol::meters::{peak_dbfs, power_dbfs, MeterKind, MeterPacket};
use ucx_protocol::regs::CHANNELS;
use ucx_protocol::word::{encode_burst, Word};
use ucx_usb::Device;

const MIXER_IF: u32 = 1;
const EP_WORDS: u8 = 0x0c;
const EP_STATUS: u8 = 0x8d;
const EP_METERS: u8 = 0x85;
/// About 240 meter packets a second, 48 of each kind.
const READ_EVERY: Duration = Duration::from_micros(4167);
const RETRY: Duration = Duration::from_secs(1);
const STATUS_EVERY: Duration = Duration::from_secs(1);
/// At most this many words a burst, this far apart (as TotalMix sends them).
const BURST: usize = 63;
const BURST_GAP: Duration = Duration::from_millis(20);

/// Rows of meters: hardware inputs, playback, hardware outputs (after their faders).
pub const ROWS: usize = 3;

/// What the window's changes may reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    /// Nothing is sent.
    Nothing,
    /// Only words for the digital outputs (SPDIF, ADAT), where nothing is plugged in.
    Digital,
    /// Everything, the analog outputs too.
    All,
}

impl Reach {
    pub fn allows(self, w: Word) -> bool {
        match self {
            Reach::Nothing => false,
            Reach::Digital => digital_only(w),
            Reach::All => true,
        }
    }
}

/// The loudest of each meter since they were last taken, dBFS (−∞ for nothing).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Levels {
    pub peak: [[f32; CHANNELS]; ROWS],
    pub rms: [[f32; CHANNELS]; ROWS],
}

impl Levels {
    pub const SILENT: Levels =
        Levels { peak: [[f32::NEG_INFINITY; CHANNELS]; ROWS], rms: [[f32::NEG_INFINITY; CHANNELS]; ROWS] };

    fn add(&mut self, p: &MeterPacket) {
        let row = match p.kind {
            MeterKind::InputsA => 0,
            MeterKind::Playback => 1,
            MeterKind::OutputsPostFader => 2,
            MeterKind::InputsB | MeterKind::OutputsPreFader => return,
        };
        for c in 0..CHANNELS {
            self.peak[row][c] = self.peak[row][c].max(peak_dbfs(p.peak[c]));
            self.rms[row][c] = self.rms[row][c].max(power_dbfs(p.power[c]));
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Searching,
    /// No unit, or it couldn't be opened: why.
    Absent(String),
    /// Its mixer interface is claimed by another program.
    Busy(String),
    Live { rate: Option<u32> },
}

struct Shared {
    status: Status,
    levels: Levels,
    fresh: bool,
    reach: Reach,
    /// Words waiting to go out, in order.
    outbox: VecDeque<Word>,
    sent: u64,
    /// Words lost: queued when the unit went away, or refused here.
    lost: u64,
}

/// The running link; stops its thread when dropped.
pub struct Unit {
    shared: Arc<Mutex<Shared>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Unit {
    /// Starts looking for the unit, sending nothing. `wake` is called when the status changes
    /// (a repaint).
    pub fn start(wake: impl Fn() + Send + 'static) -> Unit {
        let shared = Arc::new(Mutex::new(Shared {
            status: Status::Searching,
            levels: Levels::SILENT,
            fresh: false,
            reach: Reach::Nothing,
            outbox: VecDeque::new(),
            sent: 0,
            lost: 0,
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let (shared, stop) = (shared.clone(), stop.clone());
            thread::Builder::new()
                .name("ucx-link".into())
                .spawn(move || run(&shared, &stop, &wake))
                .expect("starting the unit's thread")
        };
        Unit { shared, stop, thread: Some(thread) }
    }

    pub fn status(&self) -> Status {
        lock(&self.shared).status.clone()
    }

    /// The meters' loudest since the last call, if any came in.
    pub fn take_levels(&self) -> Option<Levels> {
        let mut s = lock(&self.shared);
        if !s.fresh {
            return None;
        }
        s.fresh = false;
        Some(std::mem::replace(&mut s.levels, Levels::SILENT))
    }

    /// What may be sent from now on. Narrowing it drops queued words it no longer allows.
    pub fn set_reach(&self, reach: Reach) {
        let mut s = lock(&self.shared);
        s.reach = reach;
        let before = s.outbox.len();
        s.outbox.retain(|w| reach.allows(*w));
        s.lost += (before - s.outbox.len()) as u64;
    }

    /// Queues words to send, the ones the reach allows, while the unit is there (none are kept
    /// for later: the window holds them back itself); returns how many it took.
    pub fn send(&self, words: &[Word]) -> usize {
        let mut s = lock(&self.shared);
        if !matches!(s.status, Status::Live { .. }) {
            s.lost += words.len() as u64;
            return 0;
        }
        let reach = s.reach;
        let before = s.outbox.len();
        s.outbox.extend(words.iter().copied().filter(|w| reach.allows(*w)));
        s.outbox.len() - before
    }

    /// Words sent so far, words lost, and words still waiting.
    pub fn counts(&self) -> (u64, u64, usize) {
        let s = lock(&self.shared);
        (s.sent, s.lost, s.outbox.len())
    }
}

impl Drop for Unit {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn lock(shared: &Mutex<Shared>) -> std::sync::MutexGuard<'_, Shared> {
    shared.lock().unwrap_or_else(PoisonError::into_inner)
}

fn set_status(shared: &Mutex<Shared>, wake: &impl Fn(), status: Status) {
    let mut s = lock(shared);
    if s.status != status {
        eprintln!("ucx-mixer: {status:?}");
        s.status = status;
        drop(s);
        wake();
    }
}

/// Sleeps `d` in short steps, false if told to stop meanwhile.
fn pause(stop: &AtomicBool, d: Duration) -> bool {
    let end = Instant::now() + d;
    while Instant::now() < end {
        if stop.load(Ordering::Relaxed) {
            return false;
        }
        thread::sleep(Duration::from_millis(50));
    }
    !stop.load(Ordering::Relaxed)
}

fn run(shared: &Mutex<Shared>, stop: &AtomicBool, wake: &impl Fn()) {
    while !stop.load(Ordering::Relaxed) {
        let mut dev = match Device::open() {
            Ok(dev) => dev,
            Err(e) => {
                set_status(shared, wake, Status::Absent(e.to_string()));
                pause(stop, RETRY);
                continue;
            }
        };
        if let Err(e) = dev.claim(MIXER_IF) {
            set_status(shared, wake, Status::Busy(e.to_string()));
            pause(stop, RETRY);
            continue;
        }
        let dev = &dev;
        let (done, failed) = (AtomicBool::new(false), AtomicBool::new(false));
        let result = thread::scope(|s| {
            s.spawn(|| drain(dev, stop, &done, &failed));
            let result = link(dev, shared, stop, &failed, wake);
            done.store(true, Ordering::Relaxed);
            result
        });
        if let Err(e) = result {
            // what was waiting is lost with the connection
            let mut s = lock(shared);
            s.lost += s.outbox.len() as u64;
            s.outbox.clear();
            drop(s);
            set_status(shared, wake, Status::Absent(e.to_string()));
            pause(stop, RETRY);
        }
    }
}

/// Reads the unit's status words on `0x8d` (and lets them go: what they say isn't known yet)
/// until either flag is set, or sets `failed` when it can't: writing on with them unread
/// would stall.
fn drain(dev: &Device, stop: &AtomicBool, done: &AtomicBool, failed: &AtomicBool) {
    let mut buf = [0u8; 512];
    while !stop.load(Ordering::Relaxed) && !done.load(Ordering::Relaxed) {
        match dev.bulk(EP_STATUS, &mut buf, 100) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(_) => {
                failed.store(true, Ordering::Relaxed);
                return;
            }
        }
    }
}

/// Reads meters and sends what's queued until told to stop or the unit goes away.
fn link(dev: &Device, shared: &Mutex<Shared>, stop: &AtomicBool, failed: &AtomicBool, wake: &impl Fn()) -> io::Result<()> {
    {
        // nothing from before this connection goes out on it
        let mut s = lock(shared);
        s.lost += s.outbox.len() as u64;
        s.outbox.clear();
    }
    let mut buf = [0u8; 512];
    let mut read_at = Instant::now();
    let mut status_at = read_at;
    let mut burst_at = read_at;
    while !stop.load(Ordering::Relaxed) {
        if failed.load(Ordering::Relaxed) {
            return Err(io::Error::other("the unit's status words stopped"));
        }
        let now = Instant::now();
        if now >= status_at {
            let rate = rate_from_status(dev.read(request::STATUS)?);
            set_status(shared, wake, Status::Live { rate });
            status_at = now + STATUS_EVERY;
        }
        if now >= burst_at {
            let words: Vec<Word> = {
                let mut s = lock(shared);
                let n = s.outbox.len().min(BURST);
                let reach = s.reach;
                s.outbox.drain(..n).filter(|w| reach.allows(*w)).collect()
            };
            if !words.is_empty() {
                let mut bytes = encode_burst(&words);
                dev.bulk(EP_WORDS, &mut bytes, 1000)?;
                lock(shared).sent += words.len() as u64;
                burst_at = Instant::now() + BURST_GAP;
            }
        }
        read_at += READ_EVERY;
        let now = Instant::now();
        if read_at > now {
            thread::sleep(read_at - now);
        } else {
            read_at = now;
        }
        match dev.bulk(EP_METERS, &mut buf, 250) {
            Ok(n) => {
                if let Ok(p) = MeterPacket::parse(&buf[..n]) {
                    let mut s = lock(shared);
                    s.levels.add(&p);
                    s.fresh = true;
                }
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucx_mixer::{Mixer, Send, Source};

    #[test]
    fn reach_lets_through_what_it_says() {
        let mut m = Mixer::totalmix();
        let analog = m.set_send(0, Source::Playback(1), Send { level: 0, pan: 0 });
        let digital = m.set_send(4, Source::Playback(1), Send { level: 0, pan: 0 });
        let mute = m.set_mute(Source::Input(0), true);
        assert!(analog.iter().chain(&digital).all(|w| !Reach::Nothing.allows(*w)));
        assert!(digital.iter().all(|w| Reach::Digital.allows(*w)));
        assert!(analog.iter().all(|w| !Reach::Digital.allows(*w)));
        assert!(mute.iter().any(|w| !Reach::Digital.allows(*w)));
        assert!(analog.iter().chain(&digital).chain(&mute).all(|w| Reach::All.allows(*w)));
    }
}
