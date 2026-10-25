//! Isochronous audio on interface 0 (iso IN `0x8f`, OUT `0x0e`, one packet per microframe), as
//! measured on Linux (`re/NOTES.md`, "Isochronous streaming, interface 0").
//!
//! Samples are float32 little-endian, 1.0 = full scale, in DSP channel order (analog 1–8,
//! SPDIF L/R, ADAT 1–8; fewer ADAT channels at 2× and 4×).
//!
//! - IN: a 4-byte [`InHeader`], then the packet's frames **planar**: all of channel 1's samples,
//!   then all of channel 2's, ...
//! - OUT: frames **interleaved**, each led by a 4-byte word. The first frame's word is the frame
//!   time the packet plays at ([`write_out`], [`OUT_LEAD`]).

/// Channels per direction at speed 1, 2 or 4.
pub const fn channels(speed: u8) -> usize {
    match speed {
        1 => 18,
        2 => 14,
        _ => 12,
    }
}

/// Bytes per microframe the endpoints take at speed 1, 2 or 4, as (OUT `0x0e`, IN `0x8f`):
/// wMaxPacketSize × mult in `re/descriptors/`.
pub const fn max_packet(speed: u8) -> (usize, usize) {
    match speed {
        1 => (532, 508),
        2 => (780, 732),
        _ => (1304, 1208),
    }
}

/// The most frames an OUT packet can carry at speed 1, 2 or 4 (7 / 13 / 25).
pub const fn max_out_frames(speed: u8) -> usize {
    max_packet(speed).0 / out_len(channels(speed), 1)
}

/// The unit counts frames modulo this ([`InHeader::position`]).
pub const POSITION_WRAP: u16 = 3072;

/// The unit plays OUT frames out of a ring of this many frames, at its own frame clock (the one
/// [`InHeader::position`] counts): a packet whose position is p plays at the next frame time that
/// is p modulo the ring. So each packet has to arrive within the ring's length before it plays,
/// and early enough for the unit to take it (see [`OUT_LEAD`]).
pub const PLAY_RING: u16 = 128;

/// Where an OUT packet's first frame goes: this many frames after the unit's position in the
/// microframe the packet goes out in, i.e. [`InHeader::next_position`] of the IN packet in that
/// microframe. Whole packets played on time for 2 to 120 frames at 44.1 kHz, 4 to 116 at 96 kHz
/// and 8 to 110 at 192 kHz (`re/NOTES.md`, "OUT timing"); this is the middle of the narrowest
/// range. At 192 kHz the slack is only about 2 microframes either way, so the driver has to know
/// which microframe each OUT packet goes out in.
pub const OUT_LEAD: u16 = 60;

/// The first 4 bytes of an IN packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InHeader {
    /// Frames in this packet (bits 0–4).
    pub frames: u8,
    /// Position of the packet's first frame, modulo [`POSITION_WRAP`] (bits 5–16).
    pub position: u16,
    /// 60 MHz ticks modulo 2048 (bits 21–31): the end of the packet's frames against the
    /// microframe clock (7500 ticks). Its drift is the sample clock's drift against USB time.
    pub timestamp: u16,
}

impl InHeader {
    pub const fn parse(word: u32) -> Self {
        InHeader { frames: (word & 0x1f) as u8, position: ((word >> 5) & 0xfff) as u16, timestamp: (word >> 21) as u16 }
    }

    /// The position the next packet's header should carry.
    pub const fn next_position(self) -> u16 {
        (self.position + self.frames as u16) % POSITION_WRAP
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoError {
    /// Shorter than a header, or not a header plus whole frames.
    Length(usize),
    /// The header's frame count doesn't match the packet length.
    Frames { header: u8, length: usize },
}

/// An IN packet, borrowed.
#[derive(Clone, Copy, Debug)]
pub struct InPacket<'a> {
    pub header: InHeader,
    channels: usize,
    samples: &'a [u8],
}

impl<'a> InPacket<'a> {
    pub fn parse(packet: &'a [u8], channels: usize) -> Result<Self, IsoError> {
        let frame = channels * 4;
        if packet.len() < 4 || !(packet.len() - 4).is_multiple_of(frame) {
            return Err(IsoError::Length(packet.len()));
        }
        let header = InHeader::parse(u32::from_le_bytes(packet[..4].try_into().unwrap()));
        if header.frames as usize != (packet.len() - 4) / frame {
            return Err(IsoError::Frames { header: header.frames, length: packet.len() });
        }
        Ok(InPacket { header, channels, samples: &packet[4..] })
    }

    pub fn frames(&self) -> usize {
        self.header.frames as usize
    }

    pub fn sample(&self, channel: usize, frame: usize) -> f32 {
        let i = (channel * self.frames() + frame) * 4;
        f32::from_le_bytes(self.samples[i..i + 4].try_into().unwrap())
    }

    /// One channel's samples in this packet.
    pub fn channel(&self, channel: usize) -> impl Iterator<Item = f32> + 'a {
        let n = self.frames();
        self.samples[channel * n * 4..(channel + 1) * n * 4]
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn channels(&self) -> usize {
        self.channels
    }
}

/// Bytes in an OUT packet of `frames` frames.
pub const fn out_len(channels: usize, frames: usize) -> usize {
    frames * (channels + 1) * 4
}

/// Writes an OUT packet into `buf` and returns its length: `frames` frames of `channels` samples,
/// frame f of channel c being `sample(c, f)`, each frame led by its position (`position` + f,
/// modulo [`POSITION_WRAP`]). The unit only uses the first frame's (zeros in the others play the
/// same); writing them all keeps every frame self-describing.
pub fn write_out(
    buf: &mut [u8],
    position: u16,
    channels: usize,
    frames: usize,
    mut sample: impl FnMut(usize, usize) -> f32,
) -> usize {
    let len = out_len(channels, frames);
    for (f, frame) in buf[..len].chunks_exact_mut((channels + 1) * 4).enumerate() {
        let lead = (position as u32 + f as u32) % POSITION_WRAP as u32;
        frame[..4].copy_from_slice(&lead.to_le_bytes());
        for (c, s) in frame[4..].chunks_exact_mut(4).enumerate() {
            s.copy_from_slice(&sample(c, f).to_le_bytes());
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_fields() {
        // the first two packets with frames at 44.1 kHz (re/captures/20-iso-in-44k1)
        let a = InHeader::parse(0x6fa0_0005);
        assert_eq!(a, InHeader { frames: 5, position: 0, timestamp: 893 });
        assert_eq!(InHeader::parse(0x1cc0_00a6), InHeader { frames: 6, position: 5, timestamp: 230 });
        assert_eq!(a.next_position(), 5);
        // past the wrap: 3076 frames in
        assert_eq!(InHeader::parse(0x6fe0_0085).position, 4);
    }

    #[test]
    fn packet_lengths_match_the_descriptors() {
        // the most frames a microframe can carry, + the header, fit wMaxPacketSize x mult
        assert_eq!(4 + 7 * channels(1) * 4, max_packet(1).1);
        assert_eq!(4 + 13 * channels(2) * 4, max_packet(2).1);
        assert!(4 + 25 * channels(4) * 4 <= max_packet(4).1);
        assert_eq!(out_len(channels(1), 7), max_packet(1).0);
        assert_eq!(out_len(channels(2), 13), max_packet(2).0);
        assert!(out_len(channels(4), 25) <= max_packet(4).0);
        assert_eq!([max_out_frames(1), max_out_frames(2), max_out_frames(4)], [7, 13, 25]);
    }

    #[test]
    fn out_packet_layout() {
        let mut buf = [0u8; 532];
        let n = write_out(&mut buf, 3071, 18, 2, |c, f| (c + 10 * f) as f32);
        assert_eq!(n, 2 * 19 * 4);
        let word = |i: usize| u32::from_le_bytes(buf[i * 4..i * 4 + 4].try_into().unwrap());
        assert_eq!(word(0), 3071);
        assert_eq!(f32::from_bits(word(1)), 0.0);
        assert_eq!(f32::from_bits(word(18)), 17.0);
        assert_eq!(word(19), 0, "position wraps");
        assert_eq!(f32::from_bits(word(20)), 10.0);
    }

    #[test]
    fn rejects_bad_lengths() {
        assert_eq!(InPacket::parse(&[0; 3], 18).unwrap_err(), IsoError::Length(3));
        assert_eq!(InPacket::parse(&[0; 76], 18).unwrap_err(), IsoError::Frames { header: 0, length: 76 });
    }
}
