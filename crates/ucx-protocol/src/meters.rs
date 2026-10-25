//! Level meter packets from bulk IN `0x85` (interface 1). The host just keeps reads pending;
//! the unit answers with 244-byte packets, five kinds in rotation, about 48 of each per second.

/// Which meters a packet carries (its last 4 bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeterKind {
    /// Hardware inputs (tag `0x11111111`).
    InputsA,
    /// Playback channels (`0x22222222`).
    Playback,
    /// Outputs before the output fader (`0x33333333`).
    OutputsPreFader,
    /// Hardware inputs, second variant (`0x55555555`; presumably pre/post EQ+dynamics).
    InputsB,
    /// Outputs after the output fader (`0x66666666`).
    OutputsPostFader,
}

pub const PACKET_LEN: usize = 244;
/// Power values at or below this mean "below range".
pub const POWER_FLOOR: u64 = 31;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeterPacket {
    pub kind: MeterKind,
    /// Mean square per channel: 512 × (24-bit sample)², full scale 2^55. 20 slots, 18 used.
    pub power: [u64; 20],
    /// Peak per channel: 16 × |24-bit sample|, full scale 2^27.
    pub peak: [u32; 18],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeterError {
    Length(usize),
    UnknownTag(u32),
}

impl MeterPacket {
    pub fn parse(b: &[u8]) -> Result<Self, MeterError> {
        if b.len() != PACKET_LEN {
            return Err(MeterError::Length(b.len()));
        }
        let tag = u32::from_le_bytes(b[240..244].try_into().unwrap());
        let kind = match tag {
            0x1111_1111 => MeterKind::InputsA,
            0x2222_2222 => MeterKind::Playback,
            0x3333_3333 => MeterKind::OutputsPreFader,
            0x5555_5555 => MeterKind::InputsB,
            0x6666_6666 => MeterKind::OutputsPostFader,
            t => return Err(MeterError::UnknownTag(t)),
        };
        let mut power = [0u64; 20];
        for (i, p) in power.iter_mut().enumerate() {
            *p = u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        }
        let mut peak = [0u32; 18];
        for (i, p) in peak.iter_mut().enumerate() {
            *p = u32::from_le_bytes(b[160 + i * 4..164 + i * 4].try_into().unwrap());
        }
        Ok(MeterPacket { kind, power, peak })
    }
}

/// RMS level in dBFS from a power value (−∞ at or below [`POWER_FLOOR`]).
pub fn power_dbfs(power: u64) -> f32 {
    if power <= POWER_FLOOR {
        return f32::NEG_INFINITY;
    }
    10.0 * (power as f64 / (1u64 << 55) as f64).log10() as f32
}

/// Peak level in dBFS.
pub fn peak_dbfs(peak: u32) -> f32 {
    if peak == 0 {
        return f32::NEG_INFINITY;
    }
    20.0 * (peak as f64 / (1u64 << 27) as f64).log10() as f32
}
