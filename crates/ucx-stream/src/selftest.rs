//! Plays a position code on playback 3/4 and checks every frame of it that comes back through
//! the unit's loopback (outputs 3/4 copied into recording channels 3/4), as `out_timing.py` does.
//!
//! Each frame carries its own position q as 128 + q mod 128 and 128 + (q >> 7) mod 128 LSB of
//! 24 bits (−96 to −90 dBFS), the other channels stay silent. So the recording says, frame by
//! frame, which position played where: on time (a fixed loopback delay after its position),
//! silent, a whole number of 128-frame rings off, or something else.

use std::collections::VecDeque;

use ucx_protocol::iso::{InPacket, PLAY_RING};

use crate::engine::{Client, Stats};

/// Playback and recording channels 3 and 4, 0-based.
pub const LOOPED: [usize; 2] = [2, 3];
const LSB_PER_UNIT: f32 = (1 << 23) as f32;
const CODE: i64 = 1 << 14;

fn code(q: i64) -> [f32; 2] {
    [(128 + q.rem_euclid(128)) as f32 / LSB_PER_UNIT, (128 + (q >> 7).rem_euclid(128)) as f32 / LSB_PER_UNIT]
}

/// What a recorded frame of channels 3/4 holds.
enum Heard {
    Silence,
    Code(i64),
    Other,
}

fn decode(a: f32, b: f32) -> Heard {
    if a == 0.0 && b == 0.0 {
        return Heard::Silence;
    }
    let (a, b) = ((a * LSB_PER_UNIT).round() as i64, (b * LSB_PER_UNIT).round() as i64);
    if (128..256).contains(&a) && (128..256).contains(&b) {
        Heard::Code((b - 128) << 7 | (a - 128))
    } else {
        Heard::Other
    }
}

/// How the recorded frames came out, for positions that were played and for ones left out.
#[derive(Clone, Copy, Debug, Default)]
pub struct Counts {
    pub on_time: u64,
    pub silent: u64,
    /// A whole number of rings early or late.
    pub ring_off: u64,
    pub wrong: u64,
    pub other: u64,
}

impl Counts {
    pub fn errors(&self) -> u64 {
        self.silent + self.ring_off + self.wrong + self.other
    }
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} on time, {} silent, {} a ring off, {} wrong, {} not the code",
            self.on_time, self.silent, self.ring_off, self.wrong, self.other
        )
    }
}

pub struct Selftest {
    channels: usize,
    /// Leave every this many OUT packets empty.
    gap_every: Option<u64>,
    packets: u64,
    /// Positions handed out: from the first to the latest.
    played: Option<(i64, i64)>,
    /// Positions left out on purpose, oldest first.
    gaps: VecDeque<(i64, i64)>,
    /// Recorded position minus played position, from the first frame that came back.
    pub delay: Option<i64>,
    pub played_frames: Counts,
    pub gap_frames: Counts,
    examples: Vec<String>,
}

impl Selftest {
    pub fn new(channels: usize, gap_every: Option<u64>) -> Selftest {
        Selftest {
            channels,
            gap_every,
            packets: 0,
            played: None,
            gaps: VecDeque::new(),
            delay: None,
            played_frames: Counts::default(),
            gap_frames: Counts::default(),
            examples: Vec::new(),
        }
    }

    /// The first few frames that didn't come back on time, described.
    pub fn examples(&self) -> &[String] {
        &self.examples
    }

    fn check(&mut self, p: i64, heard: Heard) {
        let Some(delay) = self.delay else {
            if let Heard::Code(q) = heard {
                let d = (p - q).rem_euclid(CODE);
                self.delay = Some(if d >= CODE / 2 { d - CODE } else { d });
                self.played_frames.on_time += 1;
            }
            return;
        };
        let q = p - delay;
        match self.played {
            Some((from, to)) if (from..to).contains(&q) => {}
            _ => return, // not played (yet), or before the code started
        }
        while self.gaps.front().is_some_and(|&(_, to)| to <= q) {
            self.gaps.pop_front();
        }
        let in_gap = self.gaps.front().is_some_and(|&(from, _)| from <= q);
        let counts = if in_gap { &mut self.gap_frames } else { &mut self.played_frames };
        let what = match heard {
            Heard::Code(c) if c == q.rem_euclid(CODE) => {
                counts.on_time += 1;
                return;
            }
            Heard::Code(c) => {
                let d = (q - c).rem_euclid(CODE);
                let d = if d >= CODE / 2 { d - CODE } else { d };
                if d % PLAY_RING as i64 == 0 {
                    counts.ring_off += 1;
                } else {
                    counts.wrong += 1;
                }
                format!("position {q} came back as {} ({d:+} frames)", q - d)
            }
            Heard::Silence => {
                counts.silent += 1;
                format!("position {q} came back silent")
            }
            Heard::Other => {
                counts.other += 1;
                format!("position {q} came back as something else")
            }
        };
        if !in_gap && self.examples.len() < 10 {
            self.examples.push(what);
        }
    }
}

impl Client for Selftest {
    fn capture(&mut self, position: i64, packet: &InPacket) {
        let a: Vec<f32> = packet.channel(LOOPED[0]).collect();
        let b: Vec<f32> = packet.channel(LOOPED[1]).collect();
        for (f, (&a, &b)) in a.iter().zip(&b).enumerate() {
            self.check(position + f as i64, decode(a, b));
        }
    }

    fn playback(&mut self, position: i64, out: &mut [f32]) -> bool {
        let frames = (out.len() / self.channels) as i64;
        self.packets += 1;
        let from = self.played.map_or(position, |(from, _)| from);
        self.played = Some((from, position + frames));
        if self.gap_every.is_some_and(|g| self.packets.is_multiple_of(g)) {
            self.gaps.push_back((position, position + frames));
            return false;
        }
        for (f, frame) in out.chunks_exact_mut(self.channels).enumerate() {
            let [lo, hi] = code(position + f as i64);
            frame[LOOPED[0]] = lo;
            frame[LOOPED[1]] = hi;
        }
        true
    }

    fn restarted(&mut self) {
        self.delay = None;
        self.played = None;
        self.gaps.clear();
    }

    fn tick(&mut self, stats: &Stats) {
        let check = stats.offset_check.map_or(String::new(), |(lo, hi)| format!(" (measured {lo:.3}..{hi:.3})"));
        eprintln!(
            "{:.2} Hz, offset {}{check}, IN {} packets / {} errors ({}) / {} jumps, OUT {} errors ({}), \
             {} resyncs, busiest {} us",
            stats.rate,
            stats.offset.map_or("?".into(), |s| s.to_string()),
            stats.in_packets,
            stats.in_errors,
            stats.in_error_status,
            stats.position_jumps,
            stats.out_errors,
            stats.out_error_status,
            stats.resyncs,
            stats.busiest_us
        );
        eprintln!("  loopback: {}", self.played_frames);
        if self.gap_every.is_some() {
            eprintln!("  left out: {}", self.gap_frames);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trip() {
        for q in [0, 1, 127, 128, 5000, CODE - 1] {
            let [a, b] = code(q);
            assert!(matches!(decode(a, b), Heard::Code(c) if c == q));
        }
        assert!(matches!(decode(0.0, 0.0), Heard::Silence));
        assert!(matches!(decode(2.6e-6, 0.0), Heard::Other));
    }
}
