//! Channel numbering and register addresses.
//!
//! The DSP has 18 hardware inputs and 18 hardware outputs, each with a 256-register block at
//! `channel << 8`: inputs are channels `0x00`–`0x11`, outputs `0x12`–`0x23`. The order is the
//! same everywhere (ASIO, USB stream slots, TotalMix): analog 1–8, SPDIF coax L/R, ADAT 1–8.

/// Channels per direction.
pub const CHANNELS: usize = 18;

pub const INPUT_NAMES: [&str; CHANNELS] = [
    "Mic 1", "Mic 2", "AN 3", "AN 4", "AN 5", "AN 6", "AN 7", "AN 8", "SPDIF L", "SPDIF R",
    "ADAT 1", "ADAT 2", "ADAT 3", "ADAT 4", "ADAT 5", "ADAT 6", "ADAT 7", "ADAT 8",
];

pub const OUTPUT_NAMES: [&str; CHANNELS] = [
    "AN 1", "AN 2", "AN 3", "AN 4", "AN 5", "AN 6", "PH 7", "PH 8", "SPDIF L", "SPDIF R",
    "ADAT 1", "ADAT 2", "ADAT 3", "ADAT 4", "ADAT 5", "ADAT 6", "ADAT 7", "ADAT 8",
];

/// A hardware input or output, by index 0–17.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    Input(u8),
    Output(u8),
}

impl Port {
    /// The DSP channel number: inputs `0x00`–`0x11`, outputs `0x12`–`0x23`.
    pub const fn dsp_channel(self) -> u8 {
        match self {
            Port::Input(i) => i,
            Port::Output(o) => CHANNELS as u8 + o,
        }
    }
}

/// Address of a per-channel register (see [`offset`]).
pub const fn channel_reg(port: Port, offset: u8) -> u16 {
    ((port.dsp_channel() as u16) << 8) | offset as u16
}

/// Offsets inside a channel's register block. Values are 0/1 for switches unless noted; see
/// [`crate::values`] for the units.
pub mod offset {
    /// Inputs: mute (also zero the input's matrix entries). Outputs: output fader, 0.1 dB.
    pub const MUTE_OR_FADER: u8 = 0x00;
    /// Inputs: FX send level, 0.1 dB (post-fader). Outputs: balance −100…100.
    pub const FX_SEND_OR_BALANCE: u8 = 0x01;
    /// Outputs: mute.
    pub const OUTPUT_MUTE: u8 = 0x02;
    /// Outputs: FX return level, 0.1 dB.
    pub const FX_RETURN: u8 = 0x03;
    /// Inputs: M/S processing.
    pub const MS_PROC: u8 = 0x05;
    /// Inputs: phase invert (recording path; TotalMix also flips the matrix entries).
    pub const INPUT_PHASE: u8 = 0x06;
    /// Mic 1/2, AN 3/4: preamp gain, 0.1 dB. AN 5–8: reference level (one setting for all four).
    /// Outputs: phase invert.
    pub const GAIN_REFLEVEL_OR_PHASE: u8 = 0x07;
    /// Outputs: reference level (AN 1–6 share one setting).
    pub const OUTPUT_REF_LEVEL: u8 = 0x08;
    /// AN 3/4: Hi-Z (instrument) input.
    pub const HI_Z: u8 = 0x09;
    /// Mic 1/2, AN 3/4: AutoSet.
    pub const AUTOSET: u8 = 0x0a;

    pub const LOW_CUT_ON: u8 = 0x20;
    /// Hz, 20–500.
    pub const LOW_CUT_FREQ: u8 = 0x21;
    /// 0–3 = 6 / 12 / 18 / 24 dB/oct.
    pub const LOW_CUT_SLOPE: u8 = 0x22;

    pub const EQ_ON: u8 = 0x40;
    /// Band 1 type: 0 Bell, 1 Shelf.
    pub const EQ1_TYPE: u8 = 0x41;
    /// Gain (signed 0.1 dB, ±200), frequency (Hz, 20–20000), Q × 10 (7–50) for band `band` 1–3.
    /// Band 3's type sits at 0x48 (0 Bell, 1 Shelf, 2 Low Pass).
    pub const fn eq_band(band: u8) -> (u8, u8, u8) {
        match band {
            1 => (0x42, 0x43, 0x44),
            2 => (0x45, 0x46, 0x47),
            _ => (0x49, 0x4a, 0x4b),
        }
    }
    pub const EQ3_TYPE: u8 = 0x48;

    pub const DYN_ON: u8 = 0x60;
    /// Signed 0.1 dB, ±300.
    pub const DYN_GAIN: u8 = 0x61;
    /// ms, 0–200.
    pub const DYN_ATTACK: u8 = 0x62;
    /// ms, 100–999.
    pub const DYN_RELEASE: u8 = 0x63;
    /// Signed 0.1 dB, −600–0.
    pub const COMP_THRESHOLD: u8 = 0x64;
    /// Ratio × 10, 10–100.
    pub const COMP_RATIO: u8 = 0x65;
    /// Signed 0.1 dB, −990 to −300.
    pub const EXP_THRESHOLD: u8 = 0x66;
    /// Ratio × 10.
    pub const EXP_RATIO: u8 = 0x67;

    pub const AUTO_LEVEL_ON: u8 = 0x80;
    /// 0.1 dB, 0–180.
    pub const AUTO_LEVEL_MAX_GAIN: u8 = 0x81;
    /// 0.1 dB, 30–120.
    pub const AUTO_LEVEL_HEADROOM: u8 = 0x82;
    /// 0.1 s, 1–99.
    pub const AUTO_LEVEL_RISE_TIME: u8 = 0x83;
}

/// A mixer source: a hardware input or a playback (DAW) channel, by index 0–17.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    Input(u8),
    Playback(u8),
}

impl Source {
    const fn index(self) -> u16 {
        match self {
            Source::Input(i) => i as u16,
            Source::Playback(p) => 0x20 + p as u16,
        }
    }
}

/// Matrix output slots 30/31 feed the FX (reverb/echo) send bus.
pub const FX_BUS: [u8; 2] = [30, 31];

/// Address of the matrix gain from `source` to matrix output `output` (0–17 hardware outputs,
/// 30/31 FX bus). The value is a [`crate::values::MatrixGain`].
pub const fn matrix_reg(output: u8, source: Source) -> u16 {
    0x4000 + output as u16 * 0x40 + source.index()
}

/// Address of the unit's own fader/pan record for hardware input `input` in output `output`'s
/// submix. Holds two fields, see [`crate::values::FaderTableField`]. TotalMix keeps it in step
/// with the matrix; the audio gains are in the matrix.
pub const fn fader_table_reg(output: u8, input: u8) -> u16 {
    channel_reg(Port::Output(output), 0xe0 + input)
}

/// Global registers.
pub mod global {
    /// Global mute enable (1 = mute buttons active).
    pub const MUTE_ENABLE: u16 = 0x3d01;
    /// Main output Dim.
    pub const DIM: u16 = 0x3d03;
    /// Main monitor mask? `0xffff` normally, `0x9bff` with Mono. Resent with [`MONITOR_FLAGS`]
    /// about once a second.
    pub const MONITOR_MASK: u16 = 0x3f98;
    /// Main monitor flags: bit 1 Mono, bit 2 Main mute.
    pub const MONITOR_FLAGS: u16 = 0x3f9e;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_from_captures() {
        assert_eq!(channel_reg(Port::Input(0), offset::EQ_ON), 0x0040);
        assert_eq!(channel_reg(Port::Input(4), offset::eq_band(1).0), 0x0442);
        assert_eq!(channel_reg(Port::Output(16), offset::EQ_ON), 0x2240);
        // Mic 1 -> AN 3/4 and playback 1/2 -> AN 3/4, from the matrix probes
        assert_eq!(matrix_reg(2, Source::Input(0)), 0x4080);
        assert_eq!(matrix_reg(3, Source::Input(0)), 0x40c0);
        assert_eq!(matrix_reg(2, Source::Playback(0)), 0x40a0);
        assert_eq!(matrix_reg(3, Source::Playback(1)), 0x40e1);
        // playback AN 1/2 reverb send
        assert_eq!(matrix_reg(FX_BUS[0], Source::Playback(0)), 0x47a0);
        assert_eq!(fader_table_reg(0, 6), 0x12e6);
        assert_eq!(fader_table_reg(3, 1), 0x15e1);
    }
}
