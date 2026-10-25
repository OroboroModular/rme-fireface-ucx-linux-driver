//! Interface-1 register words: bits 30–16 address, bits 15–0 value, bit 31 odd parity.
//!
//! Sent little-endian on bulk OUT `0x0c` (TotalMix sends single words or bursts of up to 63 in one
//! transfer); the unit answers polls on bulk IN `0x8d` with words of the same format.

use std::fmt;

/// One register write: a 15-bit address and a 16-bit value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Word {
    pub addr: u16,
    pub value: u16,
}

/// A received word whose parity bit doesn't make the word's bit count odd.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParityError(pub u32);

impl fmt::Display for ParityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "register word {:08x} has even parity", self.0)
    }
}

impl std::error::Error for ParityError {}

impl Word {
    /// `addr` must fit in 15 bits.
    pub const fn new(addr: u16, value: u16) -> Self {
        assert!(addr < 0x8000, "register addresses are 15 bits");
        Word { addr, value }
    }

    /// The word as sent, with bit 31 set when needed to make the number of one bits odd.
    pub const fn encode(self) -> u32 {
        let w = ((self.addr as u32) << 16) | self.value as u32;
        if w.count_ones() & 1 == 0 {
            w | 0x8000_0000
        } else {
            w
        }
    }

    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.encode().to_le_bytes()
    }

    pub fn decode(raw: u32) -> Result<Self, ParityError> {
        if raw.count_ones() & 1 == 0 {
            return Err(ParityError(raw));
        }
        Ok(Word { addr: ((raw >> 16) & 0x7fff) as u16, value: raw as u16 })
    }
}

/// Bytes for one bulk transfer carrying `words`.
pub fn encode_burst(words: &[Word]) -> Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_matches_captured_polls() {
        // TotalMix's idle poll words 0x3f00|N as captured: top byte 0x3f or 0xbf by parity
        assert_eq!(Word::new(0x3f00, 0x0c).encode(), 0xbf00_000c);
        assert_eq!(Word::new(0x3f00, 0x0d).encode(), 0x3f00_000d);
        assert_eq!(Word::new(0x3f00, 0x00).encode(), 0xbf00_0000);
    }

    #[test]
    fn eq_enable_word() {
        // Mic 1 EQ on: address and value have two one bits, so the parity bit is set
        assert_eq!(Word::new(0x0040, 1).to_le_bytes(), [0x01, 0x00, 0x40, 0x80]);
        assert_eq!(Word::new(0x0040, 0).encode(), 0x0040_0000);
    }

    #[test]
    fn decode_rejects_even_parity() {
        assert!(Word::decode(0x0040_0001).is_err());
        assert_eq!(Word::decode(0x8040_0001), Ok(Word::new(0x0040, 1)));
    }
}
