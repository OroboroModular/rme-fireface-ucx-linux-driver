//! Vendor control requests on endpoint 0 (`bmRequestType` 0x40 out, 0xc0 in, no data stage for
//! writes): sample rate, clock source, device options, streaming, status.

/// Request numbers seen from the Windows driver.
pub mod request {
    /// Masked control-register write: `wValue` = bits, `wIndex` = which bits ([`super::ControlWrite`]).
    pub const CONTROL: u8 = 16;
    /// Read, 4 bytes: status word ([`super::rate_from_status`]). Polled every ~16 ms.
    pub const STATUS: u8 = 17;
    /// Write: mute playback channels ([`super::playback_mask`]). Sent when a stream stops.
    pub const MUTE: u8 = 19;
    /// Write: unmute playback channels ([`super::playback_mask`]). Sent when a stream starts;
    /// playback channels left out of the mask stay silent.
    pub const UNMUTE: u8 = 20;
    /// Write, `wIndex` 0–29: loopback (`wValue` 1) per output; the driver always sends all 30.
    pub const LOOPBACK: u8 = 21;
    /// Write, sent at power-on for `wIndex` 0x00–0x1d and 0x20–0x3d with `wValue` 0. Meaning unknown.
    pub const INIT_22: u8 = 22;
    /// Masked write (`wIndex` 0x0fc0); bit 8 mirrors control bit 8. Rest unknown.
    pub const OPTIONS_23: u8 = 23;
    /// Write, 4 × at power-on: 24-bit value as `wIndex` high byte : `wValue`, index in `wIndex`
    /// low byte. (UC project: sample rate.) Meaning unknown.
    pub const COEFF_27: u8 = 27;
    /// Read, 4 bytes, polled: constant `03 01 31 00` (UC project: firmware version).
    pub const FIRMWARE: u8 = 28;
    /// Write `0/0` at every stream start, after setting [`super::STREAM_ENABLE`].
    pub const STREAM_29: u8 = 29;
    /// Reads, 4 bytes each, polled: the serial number, low and high part.
    pub const SERIAL_LO: u8 = 30;
    pub const SERIAL_HI: u8 = 31;
}

/// A vendor OUT request without data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VendorWrite {
    pub request: u8,
    pub value: u16,
    pub index: u16,
}

/// Request 16: set the bits of `value` selected by `mask` in the control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlWrite {
    pub value: u16,
    pub mask: u16,
}

impl ControlWrite {
    pub const fn request(self) -> VendorWrite {
        VendorWrite { request: request::CONTROL, value: self.value, index: self.mask }
    }
}

/// Set at every stream start.
pub const STREAM_ENABLE: ControlWrite = ControlWrite { value: 0x8000, mask: 0x8000 };

/// Sample rate family (control bits 4–5). Speed is not a register: see [`alt_setting`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseRate {
    R32000 = 0,
    R44100 = 1,
    R48000 = 2,
}

/// The supported rates, in the order of the status word's rate index (gaps at 3 and 7).
pub const RATES: [u32; 9] = [32000, 44100, 48000, 64000, 88200, 96000, 128000, 176400, 192000];

/// Base rate family and speed (1, 2 or 4) for a supported rate.
pub fn split_rate(rate: u32) -> Option<(BaseRate, u8)> {
    let base = match rate {
        32000 | 64000 | 128000 => BaseRate::R32000,
        44100 | 88200 | 176400 => BaseRate::R44100,
        48000 | 96000 | 192000 => BaseRate::R48000,
        _ => return None,
    };
    let speed = match rate {
        32000 | 44100 | 48000 => 1,
        64000 | 88200 | 96000 => 2,
        _ => 4,
    };
    Some((base, speed))
}

pub const fn base_rate(base: BaseRate) -> ControlWrite {
    ControlWrite { value: (base as u16) << 4, mask: 0x0030 }
}

/// Streaming interface alt setting for a speed: the unit takes 1× / 2× / 4× from it
/// (alt 1 / 2 / 3, the same on interface 5 and the iso interface 0).
pub const fn alt_setting(speed: u8) -> u8 {
    match speed {
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

/// Rate from the status word (request 17, as a little-endian u32): the top byte is
/// (speed << 2) | base, the index into [`RATES`] with gaps.
pub fn rate_from_status(status: u32) -> Option<u32> {
    match status >> 24 {
        0 => Some(32000),
        1 => Some(44100),
        2 => Some(48000),
        4 => Some(64000),
        5 => Some(88200),
        6 => Some(96000),
        8 => Some(128000),
        9 => Some(176400),
        10 => Some(192000),
        _ => None,
    }
}

/// Preferred clock source (control bits 1–2). Without a valid signal there the unit stays on
/// its internal clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockSource {
    Internal = 0,
    SpdifCoax = 1,
    Optical = 2,
    WordClock = 3,
}

pub const fn clock_source(source: ClockSource) -> ControlWrite {
    ControlWrite { value: (source as u16) << 1, mask: 0x0006 }
}

/// Device options in the control register, as the settings dialog writes them (mask `0x05cf`
/// together with the clock source).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    pub clock: ClockSource,
    pub word_termination: bool,
    pub word_single_speed: bool,
    pub spdif_professional: bool,
    /// The dialog's "EQ+D for Record"; the register bit is the inverse.
    pub eq_dyn_for_record: bool,
    /// Optical output as SPDIF instead of ADAT.
    pub optical_out_spdif: bool,
}

impl Options {
    pub const fn control_write(self) -> ControlWrite {
        let mut v = (self.clock as u16) << 1;
        if self.word_termination {
            v |= 0x0008;
        }
        if self.word_single_speed {
            v |= 0x0040;
        }
        if self.spdif_professional {
            v |= 0x0080;
        }
        if !self.eq_dyn_for_record {
            v |= 0x0100;
        }
        if self.optical_out_spdif {
            v |= 0x0400;
        }
        ControlWrite { value: v, mask: 0x05cf }
    }
}

/// Request 19 or 20 for playback channels 0..`channels`: a 32-bit mask with channel n at bit
/// n + 2, high half in `wValue`, low half in `wIndex`. The Windows driver sent `0000/000c` for a
/// 2-channel stream and `000f/fffc` for all 18 (`re/captures/12-stream-test`, `12-stream-pattern`).
pub const fn playback_mask(request: u8, channels: u8) -> VendorWrite {
    let mask = ((1u32 << channels) - 1) << 2;
    VendorWrite { request, value: (mask >> 16) as u16, index: mask as u16 }
}

/// Requests the driver sends to start streaming `channels` playback channels, after selecting
/// the alt setting. Recording on the iso interface runs without them; playback needs them.
pub const fn stream_start(channels: u8) -> [VendorWrite; 3] {
    [
        STREAM_ENABLE.request(),
        VendorWrite { request: request::STREAM_29, value: 0, index: 0 },
        playback_mask(request::UNMUTE, channels),
    ]
}

/// Requests the driver sends to stop streaming (before aborting the pipes).
pub const fn stream_stop(channels: u8) -> [VendorWrite; 1] {
    [playback_mask(request::MUTE, channels)]
}

/// The driver's control writes after the unit powers on, exactly as captured
/// (`re/captures/01-init-a.pcap`; identical in run B). Several steps are not understood yet.
pub fn power_on_sequence() -> Vec<VendorWrite> {
    let w = |request, value, index| VendorWrite { request, value, index };
    let mut s = Vec::with_capacity(107);
    for bank in [0x00u16, 0x20] {
        s.extend((0..0x1e).map(|i| w(request::INIT_22, 0, bank + i)));
    }
    s.push(w(request::COEFF_27, 0xc350, 0x0000));
    s.push(w(request::COEFF_27, 0x8db8, 0xd201));
    s.push(w(request::COEFF_27, 0x8234, 0xd302));
    s.push(w(request::COEFF_27, 0x7cff, 0xf803));
    s.push(w(request::CONTROL, 0x0119, 0x05ff));
    s.push(w(request::OPTIONS_23, 0x3f00, 0x0fc0));
    for (value, index) in [(0x0000, 0x3000), (0x0000, 0x3000), (0x0800, 0x0800), (0x0000, 0x0800), (0x2000, 0x3000),
        (0x0000, 0x3000), (0x0800, 0x0800), (0x0800, 0x0800), (0x0800, 0x0800), (0x0800, 0x0800), (0x0800, 0x0800)]
    {
        s.push(w(request::CONTROL, value, index));
    }
    s.extend((0..30).map(|i| w(request::LOOPBACK, 0, i)));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates() {
        assert_eq!(base_rate(split_rate(48000).unwrap().0), ControlWrite { value: 0x0020, mask: 0x0030 });
        assert_eq!(split_rate(176400), Some((BaseRate::R44100, 4)));
        // status words as captured during the rate sweep (bytes 00 ff f0 NN)
        assert_eq!(rate_from_status(u32::from_le_bytes([0x00, 0xff, 0xf0, 0x01])), Some(44100));
        assert_eq!(rate_from_status(u32::from_le_bytes([0x00, 0xff, 0xf0, 0x0a])), Some(192000));
    }

    #[test]
    fn playback_masks_from_stream_captures() {
        // 2-channel ASIO stream (12-stream-test) and all 18 channels (12-stream-pattern)
        assert_eq!(stream_start(2)[2], VendorWrite { request: request::UNMUTE, value: 0x0000, index: 0x000c });
        assert_eq!(stream_stop(2)[0], VendorWrite { request: request::MUTE, value: 0x0000, index: 0x000c });
        assert_eq!(stream_start(18)[2], VendorWrite { request: request::UNMUTE, value: 0x000f, index: 0xfffc });
    }

    #[test]
    fn options_from_settings_dialog_captures() {
        let o = Options {
            clock: ClockSource::Internal,
            word_termination: true,
            word_single_speed: false,
            spdif_professional: false,
            eq_dyn_for_record: true,
            optical_out_spdif: false,
        };
        assert_eq!(o.control_write(), ControlWrite { value: 0x0008, mask: 0x05cf });
        assert_eq!(Options { clock: ClockSource::WordClock, ..o }.control_write().value, 0x000e);
        assert_eq!(Options { optical_out_spdif: true, ..o }.control_write().value, 0x0408);
        assert_eq!(Options { spdif_professional: true, ..o }.control_write().value, 0x0088);
        assert_eq!(Options { eq_dyn_for_record: false, ..o }.control_write().value, 0x0108);
        assert_eq!(Options { word_single_speed: true, ..o }.control_write().value, 0x0048);
    }
}
