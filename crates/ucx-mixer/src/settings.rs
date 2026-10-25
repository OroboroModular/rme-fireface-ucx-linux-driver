//! A strip's own processing, TotalMix's channel settings: low cut, a three-band EQ, dynamics
//! (compressor and expander) and auto-level, in the units the hardware takes (`re/NOTES.md`,
//! "Channel settings"; `ucx_protocol::regs::offset`). Inputs and outputs have them, playback
//! strips don't; both channels of a pair get the same.
//!
//! Like the output levels they aren't in TotalMix's push: the unit keeps them. The settings
//! probes found every strip the same and put it back so ([`Settings::TOTALMIX`]).

use serde::{Deserialize, Serialize};
use ucx_protocol::regs::offset;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LowCut {
    pub on: bool,
    /// Hz, 20–500.
    pub freq: u16,
    /// 0–3: 6, 12, 18, 24 dB an octave.
    pub slope: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Band {
    /// Band 1: 0 Bell, 1 Shelf. Band 2: Bell only (0). Band 3: 0 Bell, 1 Shelf, 2 Low Pass.
    pub kind: u8,
    /// Tenths of a dB, ±200.
    pub gain: i16,
    /// Hz, 20–20000.
    pub freq: u16,
    /// Q × 10, 7–50.
    pub q: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eq {
    pub on: bool,
    pub bands: [Band; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dynamics {
    pub on: bool,
    /// Tenths of a dB, ±300.
    pub gain: i16,
    /// ms, 0–200.
    pub attack: u16,
    /// ms, 100–999.
    pub release: u16,
    /// Tenths of a dB, −600–0.
    pub comp_threshold: i16,
    /// Ratio × 10, 10–100.
    pub comp_ratio: u8,
    /// Tenths of a dB, −990 to −300.
    pub exp_threshold: i16,
    /// Ratio × 10, 10–100.
    pub exp_ratio: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoLevel {
    pub on: bool,
    /// Tenths of a dB, 0–180.
    pub max_gain: u16,
    /// Tenths of a dB, 30–120.
    pub headroom: u16,
    /// Tenths of a second, 1–99.
    pub rise: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub low_cut: LowCut,
    pub eq: Eq,
    pub dynamics: Dynamics,
    pub auto_level: AutoLevel,
}

/// How many registers a strip's settings take.
pub const REGISTERS: usize = 27;

impl Settings {
    /// Every strip as the settings probes found it and left it (`re/maps/channel-settings.md`,
    /// the values each parameter was put back to, the same on all 20 strips): everything off.
    pub const TOTALMIX: Settings = Settings {
        low_cut: LowCut { on: false, freq: 20, slope: 1 },
        eq: Eq {
            on: false,
            bands: [
                Band { kind: 1, gain: 0, freq: 80, q: 10 },
                Band { kind: 0, gain: 0, freq: 1000, q: 10 },
                Band { kind: 1, gain: 0, freq: 5000, q: 10 },
            ],
        },
        dynamics: Dynamics {
            on: false,
            gain: 0,
            attack: 10,
            release: 300,
            comp_threshold: -300,
            comp_ratio: 10,
            exp_threshold: -600,
            exp_ratio: 10,
        },
        auto_level: AutoLevel { on: false, max_gain: 60, headroom: 60, rise: 50 },
    };

    /// Every parameter in the range the hardware takes.
    pub fn clamped(self) -> Settings {
        let band = |b: Band, kinds: u8| Band {
            kind: b.kind.min(kinds),
            gain: b.gain.clamp(-200, 200),
            freq: b.freq.clamp(20, 20000),
            q: b.q.clamp(7, 50),
        };
        let (l, e, d, a) = (self.low_cut, self.eq, self.dynamics, self.auto_level);
        Settings {
            low_cut: LowCut { on: l.on, freq: l.freq.clamp(20, 500), slope: l.slope.min(3) },
            eq: Eq { on: e.on, bands: [band(e.bands[0], 1), band(e.bands[1], 0), band(e.bands[2], 2)] },
            dynamics: Dynamics {
                on: d.on,
                gain: d.gain.clamp(-300, 300),
                attack: d.attack.min(200),
                release: d.release.clamp(100, 999),
                comp_threshold: d.comp_threshold.clamp(-600, 0),
                comp_ratio: d.comp_ratio.clamp(10, 100),
                exp_threshold: d.exp_threshold.clamp(-990, -300),
                exp_ratio: d.exp_ratio.clamp(10, 100),
            },
            auto_level: AutoLevel {
                on: a.on,
                max_gain: a.max_gain.min(180),
                headroom: a.headroom.clamp(30, 120),
                rise: a.rise.clamp(1, 99),
            },
        }
    }

    /// (register offset, value) for every parameter.
    pub fn registers(&self) -> [(u8, u16); REGISTERS] {
        let (l, e, d, a) = (self.low_cut, self.eq, self.dynamics, self.auto_level);
        let [b1, b2, b3] = e.bands;
        let (g1, f1, q1) = offset::eq_band(1);
        let (g2, f2, q2) = offset::eq_band(2);
        let (g3, f3, q3) = offset::eq_band(3);
        [
            (offset::LOW_CUT_ON, l.on as u16),
            (offset::LOW_CUT_FREQ, l.freq),
            (offset::LOW_CUT_SLOPE, l.slope as u16),
            (offset::EQ_ON, e.on as u16),
            (offset::EQ1_TYPE, b1.kind as u16),
            (g1, b1.gain as u16),
            (f1, b1.freq),
            (q1, b1.q as u16),
            (g2, b2.gain as u16),
            (f2, b2.freq),
            (q2, b2.q as u16),
            (offset::EQ3_TYPE, b3.kind as u16),
            (g3, b3.gain as u16),
            (f3, b3.freq),
            (q3, b3.q as u16),
            (offset::DYN_ON, d.on as u16),
            (offset::DYN_GAIN, d.gain as u16),
            (offset::DYN_ATTACK, d.attack),
            (offset::DYN_RELEASE, d.release),
            (offset::COMP_THRESHOLD, d.comp_threshold as u16),
            (offset::COMP_RATIO, d.comp_ratio as u16),
            (offset::EXP_THRESHOLD, d.exp_threshold as u16),
            (offset::EXP_RATIO, d.exp_ratio as u16),
            (offset::AUTO_LEVEL_ON, a.on as u16),
            (offset::AUTO_LEVEL_MAX_GAIN, a.max_gain),
            (offset::AUTO_LEVEL_HEADROOM, a.headroom),
            (offset::AUTO_LEVEL_RISE_TIME, a.rise),
        ]
    }

    /// The parameter at register `off` set to `value`; false if `off` isn't one of these.
    pub fn set_register(&mut self, off: u8, value: u16) -> bool {
        let s = self;
        let i = value as i16;
        let (g1, f1, q1) = offset::eq_band(1);
        let (g2, f2, q2) = offset::eq_band(2);
        let (g3, f3, q3) = offset::eq_band(3);
        match off {
            offset::LOW_CUT_ON => s.low_cut.on = value != 0,
            offset::LOW_CUT_FREQ => s.low_cut.freq = value,
            offset::LOW_CUT_SLOPE => s.low_cut.slope = value as u8,
            offset::EQ_ON => s.eq.on = value != 0,
            offset::EQ1_TYPE => s.eq.bands[0].kind = value as u8,
            offset::EQ3_TYPE => s.eq.bands[2].kind = value as u8,
            o if o == g1 => s.eq.bands[0].gain = i,
            o if o == f1 => s.eq.bands[0].freq = value,
            o if o == q1 => s.eq.bands[0].q = value as u8,
            o if o == g2 => s.eq.bands[1].gain = i,
            o if o == f2 => s.eq.bands[1].freq = value,
            o if o == q2 => s.eq.bands[1].q = value as u8,
            o if o == g3 => s.eq.bands[2].gain = i,
            o if o == f3 => s.eq.bands[2].freq = value,
            o if o == q3 => s.eq.bands[2].q = value as u8,
            offset::DYN_ON => s.dynamics.on = value != 0,
            offset::DYN_GAIN => s.dynamics.gain = i,
            offset::DYN_ATTACK => s.dynamics.attack = value,
            offset::DYN_RELEASE => s.dynamics.release = value,
            offset::COMP_THRESHOLD => s.dynamics.comp_threshold = i,
            offset::COMP_RATIO => s.dynamics.comp_ratio = value as u8,
            offset::EXP_THRESHOLD => s.dynamics.exp_threshold = i,
            offset::EXP_RATIO => s.dynamics.exp_ratio = value as u8,
            offset::AUTO_LEVEL_ON => s.auto_level.on = value != 0,
            offset::AUTO_LEVEL_MAX_GAIN => s.auto_level.max_gain = value,
            offset::AUTO_LEVEL_HEADROOM => s.auto_level.headroom = value,
            offset::AUTO_LEVEL_RISE_TIME => s.auto_level.rise = value,
            _ => return false,
        }
        true
    }

    /// The parameters that differ from `old`, as (offset, value).
    pub fn changed_from(&self, old: &Settings) -> Vec<(u8, u16)> {
        self.registers().into_iter().zip(old.registers()).filter(|(n, o)| n != o).map(|(n, _)| n).collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::TOTALMIX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_and_set_register_agree() {
        let mut s = Settings::TOTALMIX;
        s.eq.bands[2] = Band { kind: 2, gain: -35, freq: 12000, q: 22 };
        s.dynamics.comp_threshold = -450;
        s.auto_level.on = true;
        let mut back = Settings::TOTALMIX;
        for (off, value) in s.registers() {
            assert!(back.set_register(off, value), "{off:02x}");
        }
        assert_eq!(back, s);
        assert!(!back.set_register(0x07, 0), "preamp gain isn't one of them");
    }

    #[test]
    fn clamped_keeps_the_hardware_ranges() {
        let mut s = Settings::TOTALMIX;
        s.low_cut.freq = 5;
        s.eq.bands[1].kind = 2;
        s.eq.bands[0].kind = 2;
        s.eq.bands[2].q = 200;
        s.dynamics.exp_threshold = 0;
        s.auto_level.rise = 0;
        let c = s.clamped();
        assert_eq!((c.low_cut.freq, c.eq.bands[1].kind, c.eq.bands[0].kind, c.eq.bands[2].q), (20, 0, 1, 50));
        assert_eq!((c.dynamics.exp_threshold, c.auto_level.rise), (-300, 1));
        assert_eq!(Settings::TOTALMIX.clamped(), Settings::TOTALMIX);
    }
}
