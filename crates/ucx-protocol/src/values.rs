//! Value encodings used in the register words.
//!
//! Most settings are plain engineering units (Hz, ms, ratio × 10, signed tenths of a dB); the
//! curve mapping from knob position to value lives in the host software. Two values are special:
//! the matrix gain and the fader/pan table entries.

/// Levels at or below this many tenths of a dB mean "−∞" (fader all the way down).
pub const MINUS_INF_TENTHS: i16 = -650;

/// dB → signed tenths of a dB as sent (faders, FX send/return, EQ and dynamics gains).
/// −∞ and anything below −65 dB become [`MINUS_INF_TENTHS`].
pub fn db_to_tenths(db: f32) -> u16 {
    if db.is_nan() || db <= MINUS_INF_TENTHS as f32 / 10.0 {
        return MINUS_INF_TENTHS as u16;
    }
    ((db * 10.0).round() as i16) as u16
}

/// Signed tenths of a dB → dB, with [`MINUS_INF_TENTHS`] and below as −∞.
pub fn tenths_to_db(v: u16) -> f32 {
    let t = v as i16;
    if t <= MINUS_INF_TENTHS {
        f32::NEG_INFINITY
    } else {
        t as f32 / 10.0
    }
}

/// A mixer matrix coefficient (linear gain, negative = phase inverted).
///
/// 16 bits: bit 15 picks the scale, bits 14–0 are a signed 15-bit number n. Bit 15 clear:
/// gain = n / 32768 (−0.5 … 0.49997, fine steps). Bit 15 set: gain = n / 4096 (up to ±4,
/// about +12 dB). The fine scale is used whenever the gain fits in it, as TotalMix does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MatrixGain(pub u16);

impl MatrixGain {
    pub const ZERO: MatrixGain = MatrixGain(0);
    pub const UNITY: MatrixGain = MatrixGain(0x9000);

    pub fn from_linear(gain: f64) -> Self {
        let fine = (gain * 32768.0).round();
        if (-16384.0..=16383.0).contains(&fine) {
            return MatrixGain((fine as i16 as u16) & 0x7fff);
        }
        let coarse = (gain * 4096.0).round().clamp(-16384.0, 16383.0);
        MatrixGain(0x8000 | ((coarse as i16 as u16) & 0x7fff))
    }

    /// dB → gain; −∞ (or NaN) → [`MatrixGain::ZERO`].
    pub fn from_db(db: f64) -> Self {
        if db.is_nan() || db == f64::NEG_INFINITY {
            return Self::ZERO;
        }
        Self::from_linear(10f64.powf(db / 20.0))
    }

    pub fn linear(self) -> f64 {
        let raw = self.0 & 0x7fff;
        let n = if raw & 0x4000 != 0 { raw as i32 - 0x8000 } else { raw as i32 };
        n as f64 / if self.0 & 0x8000 != 0 { 4096.0 } else { 32768.0 }
    }

    pub fn db(self) -> f64 {
        20.0 * self.linear().abs().log10()
    }
}

/// One field of a fader/pan table entry ([`crate::regs::fader_table_reg`]). Both fields go to the
/// same address; bit 3 of the value tells them apart, the number sits in bits 15–4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaderTableField {
    /// Fader level in tenths of a dB ([`MINUS_INF_TENTHS`] = −∞).
    Level(i16),
    /// Pan, −100 (left) … +100 (right).
    Pan(i16),
}

impl FaderTableField {
    pub const fn encode(self) -> u16 {
        match self {
            FaderTableField::Level(t) => (t << 4) as u16,
            FaderTableField::Pan(p) => ((p << 4) as u16) | 0x8,
        }
    }

    pub const fn decode(v: u16) -> Self {
        let n = (v as i16) >> 4;
        if v & 0x8 != 0 {
            FaderTableField::Pan(n)
        } else {
            FaderTableField::Level(n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_values_from_captures() {
        assert_eq!(MatrixGain::UNITY.linear(), 1.0);
        // existing AN 7/8 -> AN 1/2 route at -10.6 dB, and the same phase-inverted
        assert!((MatrixGain(0x258f).db() + 10.65).abs() < 0.01);
        assert!((MatrixGain(0x5a71).linear() + 0.2934).abs() < 0.0001);
        assert_eq!(MatrixGain(0xf000).linear(), -1.0);
        assert_eq!(MatrixGain(0x4000).linear(), -0.5); // M/S
        assert_eq!(MatrixGain(0x8800).linear(), 0.5); // width 0
        assert_eq!(MatrixGain::from_linear(0.5), MatrixGain(0x8800));
        assert_eq!(MatrixGain::from_linear(-0.5), MatrixGain(0x4000));
        assert_eq!(MatrixGain::from_linear(-1.0), MatrixGain(0xf000));
        assert_eq!(MatrixGain::from_db(f64::NEG_INFINITY), MatrixGain::ZERO);
    }

    #[test]
    fn tenths() {
        assert_eq!(db_to_tenths(-32.6), 0xfeba);
        assert_eq!(db_to_tenths(6.0), 0x003c);
        assert_eq!(db_to_tenths(f32::NEG_INFINITY), 0xfd76);
        assert_eq!(tenths_to_db(0xfd76), f32::NEG_INFINITY);
        assert_eq!(tenths_to_db(0xff1a), -23.0);
    }

    #[test]
    fn fader_table_fields_from_captures() {
        assert_eq!(FaderTableField::decode(0xf960), FaderTableField::Level(-106));
        assert_eq!(FaderTableField::decode(0xd760), FaderTableField::Level(-650));
        assert_eq!(FaderTableField::decode(0xf9c8), FaderTableField::Pan(-100));
        assert_eq!(FaderTableField::decode(0x0648), FaderTableField::Pan(100));
        assert_eq!(FaderTableField::decode(0x0008), FaderTableField::Pan(0));
        assert_eq!(FaderTableField::Level(-106).encode(), 0xf960);
        assert_eq!(FaderTableField::Pan(-100).encode(), 0xf9c8);
    }
}
