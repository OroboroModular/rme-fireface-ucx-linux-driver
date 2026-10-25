//! Protocol of the RME Fireface UCX in USB vendor mode (`2a39:3fc9`), as reverse engineered from
//! USB captures of the unit and its Windows driver (see `re/NOTES.md`). No USB access here: this
//! crate builds and parses the bytes, a transport (libusb, a kernel driver) moves them.
//!
//! - [`word`]: the 32-bit register words on interface 1 (bulk `0x0c` out, `0x8d` in).
//! - [`regs`]: channel numbering and register addresses (channel settings, mixer matrix, ...).
//! - [`values`]: how values are encoded (0.1 dB, matrix gains, fader/pan table fields).
//! - [`control`]: vendor control requests on endpoint 0 (rate, clock, options, streaming).
//! - [`meters`]: level meter packets from bulk `0x85`.
//! - [`iso`]: audio packets on the isochronous interface 0.

pub mod control;
pub mod iso;
pub mod meters;
pub mod regs;
pub mod values;
pub mod word;

pub use word::Word;
