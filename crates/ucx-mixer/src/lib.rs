//! The UCX's mixer as TotalMix models it (`re/NOTES.md`, from "Interface 1 word format" on):
//! strips of hardware inputs, playback channels and hardware outputs; for each output strip a
//! submix, in which every input and playback strip has a level and a pan; and the register words
//! that put a state into the unit (`ucx_protocol`'s matrix, fader table and channel registers).
//!
//! The unit can't be read back: TotalMix keeps the mix and pushes it. [`Mixer::from_words`] reads
//! such a push, and [`totalmix_state`] is the one captured after power-on. Levels are tenths of a
//! dB, as in the unit's fader table, from [`LEVEL_OFF`] (−∞) to [`LEVEL_MAX`] (+6 dB).
//!
//! Each input and output strip also has its own processing (EQ, dynamics, low cut,
//! auto-level: [`settings`]), which the unit keeps like its output levels.
//!
//! Gains: a mono input panned centre gets −3 dB on each side, and a stereo strip at balance centre
//! goes side to side at its level, with no cross terms (both captured). A stereo strip's balance
//! is linear, as TotalMix's: at R50 the left side is at half (−6 dB, `re/maps/audible-rest.md`,
//! AN 7/8). A mono strip's pan away from the centre isn't captured (the probes ran with the
//! faders at −∞): it uses the constant-power curve that gives the captured centre.
//!
//! The unit keeps its output levels and the preamps' gains itself (its front knob and encoder
//! change them), so a push of the whole state ([`Mixer::words`]) leaves them out: they're only
//! sent when they're set.

pub mod settings;

pub use settings::Settings;

use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;
use std::ops::Range;

use serde::{Deserialize, Serialize};
use ucx_protocol::regs::{self, channel_reg, fader_table_reg, matrix_reg, offset, Port};
use ucx_protocol::values::{FaderTableField, MatrixGain, MINUS_INF_TENTHS};
use ucx_protocol::Word;

/// A strip: one channel, or a stereo pair from `first`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Strip {
    pub name: &'static str,
    pub first: u8,
    pub stereo: bool,
}

impl Strip {
    const fn mono(name: &'static str, first: u8) -> Strip {
        Strip { name, first, stereo: false }
    }

    const fn pair(name: &'static str, first: u8) -> Strip {
        Strip { name, first, stereo: true }
    }

    pub fn channels(self) -> Range<u8> {
        self.first..self.first + 1 + self.stereo as u8
    }
}

/// TotalMix's hardware input strips on the UCX: the four front inputs mono, the rest in pairs.
pub const INPUTS: [Strip; 11] = [
    Strip::mono("Mic 1", 0),
    Strip::mono("Mic 2", 1),
    Strip::mono("AN 3", 2),
    Strip::mono("AN 4", 3),
    Strip::pair("AN 5/6", 4),
    Strip::pair("AN 7/8", 6),
    Strip::pair("SPDIF", 8),
    Strip::pair("ADAT 1/2", 10),
    Strip::pair("ADAT 3/4", 12),
    Strip::pair("ADAT 5/6", 14),
    Strip::pair("ADAT 7/8", 16),
];

const PAIRS: [Strip; 9] = [
    Strip::pair("AN 1/2", 0),
    Strip::pair("AN 3/4", 2),
    Strip::pair("AN 5/6", 4),
    Strip::pair("PH 7/8", 6),
    Strip::pair("SPDIF", 8),
    Strip::pair("ADAT 1/2", 10),
    Strip::pair("ADAT 3/4", 12),
    Strip::pair("ADAT 5/6", 14),
    Strip::pair("ADAT 7/8", 16),
];

/// Playback strips: the computer's channels, in pairs named after the outputs they go to.
pub const PLAYBACK: [Strip; 9] = PAIRS;

/// Hardware output strips, each with its own submix.
pub const OUTPUTS: [Strip; 9] = PAIRS;

/// Levels in tenths of a dB: this and below is −∞ (fader all the way down).
pub const LEVEL_OFF: i16 = MINUS_INF_TENTHS;
/// +6 dB, the top of TotalMix's faders.
pub const LEVEL_MAX: i16 = 60;

/// Linear gain at `level` (tenths of a dB).
pub fn gain(level: i16) -> f64 {
    if level <= LEVEL_OFF {
        0.0
    } else {
        10f64.powf(level as f64 / 200.0)
    }
}

/// The level (tenths of a dB) nearest a linear gain, at most [`LEVEL_MAX`].
pub fn level_of(gain: f64) -> i16 {
    if gain <= 0.0 {
        return LEVEL_OFF;
    }
    (200.0 * gain.log10()).round().clamp(LEVEL_OFF as f64, LEVEL_MAX as f64) as i16
}

/// A mono strip's two sides at `pan` (−100 left … +100 right): constant power, −3 dB each at the
/// centre.
pub fn pan_sides(pan: i16) -> (f64, f64) {
    let a = (pan.clamp(-100, 100) as f64 + 100.0) / 200.0 * FRAC_PI_2;
    (a.cos(), a.sin())
}

/// A stereo strip's two sides at balance `pan`: both at the level at the centre, the side it
/// turns away from falling linearly, as TotalMix's (half at 50).
pub fn balance_sides(pan: i16) -> (f64, f64) {
    let fade = |p: i16| 1.0 - p.clamp(0, 100) as f64 / 100.0;
    (fade(pan), fade(-pan))
}

/// The pan whose sides are in the ratio of `l` to `r` ([`pan_sides`]).
fn pan_of(l: f64, r: f64) -> i16 {
    if l == 0.0 && r == 0.0 {
        return 0;
    }
    (r.atan2(l) / FRAC_PI_2 * 200.0 - 100.0).round() as i16
}

/// The balance whose sides are in the ratio of `l` to `r` ([`balance_sides`]).
fn balance_of(l: f64, r: f64) -> i16 {
    let away = |faded: f64, full: f64| ((1.0 - (faded / full).clamp(0.0, 1.0)) * 100.0).round() as i16;
    if l > r {
        -away(r, l)
    } else if r > l {
        away(l, r)
    } else {
        0
    }
}

/// An input or playback strip, by its index in [`INPUTS`] or [`PLAYBACK`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    Input(usize),
    Playback(usize),
}

impl Source {
    pub fn strip(self) -> Strip {
        match self {
            Source::Input(i) => INPUTS[i],
            Source::Playback(p) => PLAYBACK[p],
        }
    }

    fn matrix(self, channel: u8) -> regs::Source {
        match self {
            Source::Input(_) => regs::Source::Input(channel),
            Source::Playback(_) => regs::Source::Playback(channel),
        }
    }
}

/// A strip's place in one submix: its level (tenths of a dB) and its pan (mono) or balance
/// (stereo), −100 … +100.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Send {
    pub level: i16,
    pub pan: i16,
}

impl Send {
    pub const OFF: Send = Send { level: LEVEL_OFF, pan: 0 };

    fn clamped(self) -> Send {
        Send { level: self.level.clamp(LEVEL_OFF, LEVEL_MAX), pan: self.pan.clamp(-100, 100) }
    }
}

/// An output strip. Its level is the unit's own (its front knob changes it too) and isn't in
/// TotalMix's push, so it's unknown until set here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub level: Option<i16>,
    pub mute: bool,
}

/// The preamps of the four front inputs (Mic 1/2, AN 3/4; input strips 0–3): gain, Hi-Z (AN 3/4
/// only) and AutoSet. The unit keeps the gain (its front encoder sets it too), so it's unknown
/// until set here; Hi-Z and AutoSet start off, as the probes found them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preamp {
    /// Tenths of a dB: 0–650 on Mic 1/2, 0–160 on AN 3/4.
    pub gain: Option<i16>,
    pub hi_z: bool,
    pub autoset: bool,
}

/// A strip's stereo image, the same in every submix: each channel's phase, and a stereo strip's
/// width (100 as it comes, 0 mono, −100 sides swapped). The matrix carries both: TotalMix wrote
/// 0.5 to all four cross-points for width 0, and negative gains for phase. An input's phase is
/// also its channel register `0x06` (the recording path).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub phase: [bool; 2],
    pub width: i16,
}

impl Image {
    pub const PLAIN: Image = Image { phase: [false; 2], width: 100 };

    /// Each side's share of its own channel and of the other: (straight, crossed).
    pub fn spread(self) -> (f64, f64) {
        let w = self.width.clamp(-100, 100) as f64 / 100.0;
        ((1.0 + w) / 2.0, (1.0 - w) / 2.0)
    }

    fn sign(self, k: usize) -> f64 {
        if self.phase[k] {
            -1.0
        } else {
            1.0
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Image::PLAIN
    }
}

/// How many input strips have a preamp (the first ones).
pub const PREAMPS: usize = 4;

impl Preamp {
    pub const FOUND: Preamp = Preamp { gain: None, hi_z: false, autoset: false };

    /// The most gain (tenths of a dB) the preamp of input strip `i` takes: Mic 1/2 0–65 dB
    /// (captured), AN 3/4 0–16 dB (one capture: 12 dB at three quarters of TotalMix's travel).
    pub fn max_gain(i: usize) -> i16 {
        if i < 2 {
            650
        } else {
            160
        }
    }

    /// Only AN 3/4 have Hi-Z.
    pub fn has_hi_z(i: usize) -> bool {
        (2..PREAMPS).contains(&i)
    }
}

/// The whole mix: for each output strip's submix, every input and playback strip's send; the
/// strips' mutes; the outputs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mixer {
    pub inputs: [[Send; INPUTS.len()]; OUTPUTS.len()],
    pub playback: [[Send; PLAYBACK.len()]; OUTPUTS.len()],
    pub input_mute: [bool; INPUTS.len()],
    pub playback_mute: [bool; PLAYBACK.len()],
    pub outputs: [Output; OUTPUTS.len()],
    /// The strips' own processing (state saved before there was any: as TotalMix left it).
    #[serde(default = "totalmix_input_settings")]
    pub input_settings: [Settings; INPUTS.len()],
    #[serde(default = "totalmix_output_settings")]
    pub output_settings: [Settings; OUTPUTS.len()],
    #[serde(default = "found_preamps")]
    pub preamps: [Preamp; PREAMPS],
    #[serde(default = "plain_input_images")]
    pub input_image: [Image; INPUTS.len()],
    #[serde(default = "plain_playback_images")]
    pub playback_image: [Image; PLAYBACK.len()],
}

fn plain_input_images() -> [Image; INPUTS.len()] {
    [Image::PLAIN; INPUTS.len()]
}

fn plain_playback_images() -> [Image; PLAYBACK.len()] {
    [Image::PLAIN; PLAYBACK.len()]
}

fn found_preamps() -> [Preamp; PREAMPS] {
    [Preamp::FOUND; PREAMPS]
}

/// A preamp's registers on its channel (input strips 0–3 are mono: strip i is channel i).
fn preamp_registers(i: usize, p: &Preamp) -> Vec<(u16, u16)> {
    let c = i as u8;
    let mut r = Vec::new();
    if let Some(g) = p.gain {
        r.push((channel_reg(Port::Input(c), offset::GAIN_REFLEVEL_OR_PHASE), g as u16));
    }
    if Preamp::has_hi_z(i) {
        r.push((channel_reg(Port::Input(c), offset::HI_Z), p.hi_z as u16));
    }
    r.push((channel_reg(Port::Input(c), offset::AUTOSET), p.autoset as u16));
    r
}

fn totalmix_input_settings() -> [Settings; INPUTS.len()] {
    [Settings::TOTALMIX; INPUTS.len()]
}

fn totalmix_output_settings() -> [Settings; OUTPUTS.len()] {
    [Settings::TOTALMIX; OUTPUTS.len()]
}

/// The words TotalMix pushed after the unit powered on (`re/captures/01-init-a.pcap`, 2026-10-23,
/// exported for `ucx-protocol`'s tests): the mix the unit holds, unless something changed it
/// since.
pub fn totalmix_state() -> Vec<Word> {
    include_str!("../../ucx-protocol/tests/fixtures/state_push.txt")
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| Word::decode(u32::from_str_radix(l.trim(), 16).expect("hex word")).expect("odd parity"))
        .collect()
}

/// The output strips whose submixes and levels can be written where nothing can be heard on this
/// setup: SPDIF and ADAT (nothing plugged in). The first four (the analog outputs, the monitors
/// and phones) only reach the unit when that's been allowed (`ucx-mixer`'s second switch).
pub const DIGITAL_OUTPUTS: std::ops::Range<usize> = 4..9;

/// True for words that only reach the digital outputs ([`DIGITAL_OUTPUTS`]): matrix rows of
/// hardware outputs 8–17 (not the FX bus), their fader table entries, their output level and
/// mute registers, and their own processing ([`Settings`]). Everything else (the analog
/// submixes, input settings and mutes, the FX bus, global registers) can change what the analog
/// outputs play.
pub fn digital_only(w: Word) -> bool {
    let a = w.addr;
    if is_matrix(a) {
        return (8..18).contains(&((a - 0x4000) / 0x40));
    }
    let (channel, off) = (a >> 8, (a & 0xff) as u8);
    let processing = Settings::TOTALMIX.registers().iter().any(|&(o, _)| o == off);
    (0x1a..=0x23).contains(&channel)
        && (off == offset::MUTE_OR_FADER || off == offset::OUTPUT_MUTE || (0xe0..0xf2).contains(&off) || processing)
}

fn is_matrix(addr: u16) -> bool {
    (0x4000..0x4800).contains(&addr)
}

fn is_fader_table(addr: u16) -> bool {
    let (channel, off) = (addr >> 8, addr & 0xff);
    (0x12..=0x23).contains(&channel) && (0xe0..0xe0 + regs::CHANNELS as u16).contains(&off)
}

/// A strip's settings registers that differ between `old` and `new`, on each of its channels.
fn settings_words(strip: Strip, port: fn(u8) -> Port, old: &Settings, new: &Settings) -> Vec<Word> {
    let changed = new.changed_from(old);
    strip.channels().flat_map(|c| changed.iter().map(move |&(off, v)| Word::new(channel_reg(port(c), off), v))).collect()
}

/// All of a strip's settings registers, on each of its channels.
fn all_settings_words(strip: Strip, port: fn(u8) -> Port, s: &Settings) -> Vec<Word> {
    let registers = s.registers();
    strip.channels().flat_map(|c| registers.iter().map(move |&(off, v)| Word::new(channel_reg(port(c), off), v))).collect()
}

/// The same value in a channel register of both of an output strip's channels.
fn output_words(output: usize, offset: u8, value: u16) -> Vec<Word> {
    OUTPUTS[output].channels().map(|c| Word::new(channel_reg(Port::Output(c), offset), value)).collect()
}

fn matrix_word(output: u8, source: regs::Source, gain: f64) -> Word {
    Word::new(matrix_reg(output, source), MatrixGain::from_linear(gain).0)
}

impl Mixer {
    /// Everything off, outputs unknown.
    pub fn silent() -> Mixer {
        Mixer {
            inputs: [[Send::OFF; INPUTS.len()]; OUTPUTS.len()],
            playback: [[Send::OFF; PLAYBACK.len()]; OUTPUTS.len()],
            input_mute: [false; INPUTS.len()],
            playback_mute: [false; PLAYBACK.len()],
            outputs: [Output { level: None, mute: false }; OUTPUTS.len()],
            input_settings: totalmix_input_settings(),
            output_settings: totalmix_output_settings(),
            preamps: found_preamps(),
            input_image: plain_input_images(),
            playback_image: plain_playback_images(),
        }
    }

    /// The mix TotalMix left in the unit ([`totalmix_state`]).
    pub fn totalmix() -> Mixer {
        Mixer::from_words(totalmix_state())
    }

    /// The mix a push of words sets, later words over earlier ones. Inputs take their levels and
    /// pans from the fader table where it has them, everything else comes from the matrix
    /// (phase is ignored), and output strips from their channel registers.
    pub fn from_words(words: impl IntoIterator<Item = Word>) -> Mixer {
        let mut matrix = HashMap::new();
        let mut levels = HashMap::new();
        let mut pans = HashMap::new();
        let mut channel = HashMap::new();
        for w in words {
            if is_matrix(w.addr) {
                matrix.insert(w.addr, MatrixGain(w.value).linear());
            } else if is_fader_table(w.addr) {
                match FaderTableField::decode(w.value) {
                    FaderTableField::Level(t) => levels.insert(w.addr, t.clamp(LEVEL_OFF, LEVEL_MAX)),
                    FaderTableField::Pan(p) => pans.insert(w.addr, p.clamp(-100, 100)),
                };
            } else {
                channel.insert(w.addr, w.value);
            }
        }
        let signed = |output: u8, source: regs::Source| matrix.get(&matrix_reg(output, source)).copied().unwrap_or(0.0);
        let g = |output: u8, source: regs::Source| signed(output, source).abs();
        // a stereo strip's sides into a submix's (left, right): straight and crossed parts summed
        let sides = |ol: u8, or: u8, c: u8, source: fn(u8) -> regs::Source| {
            (g(ol, source(c)) + g(ol, source(c + 1)), g(or, source(c + 1)) + g(or, source(c)))
        };

        let mut m = Mixer::silent();
        for (submix, out) in OUTPUTS.iter().enumerate() {
            let (ol, or) = (out.first, out.first + 1);
            for (i, strip) in INPUTS.iter().enumerate() {
                let c = strip.first;
                let table = fader_table_reg(ol, c);
                m.inputs[submix][i] = if strip.stereo {
                    let (l, r) = sides(ol, or, c, regs::Source::Input);
                    // (each channel's entry is at what its side of the balance leaves it)
                    let table = [table, fader_table_reg(ol, c + 1)].map(|t| levels.get(&t).copied());
                    let level = match table {
                        [Some(a), Some(b)] => a.max(b),
                        [Some(a), None] | [None, Some(a)] => a,
                        [None, None] => level_of(l.max(r)),
                    };
                    Send { level, pan: balance_of(l, r) }
                } else {
                    let (l, r) = (g(ol, regs::Source::Input(c)), g(or, regs::Source::Input(c)));
                    Send {
                        level: levels.get(&table).copied().unwrap_or(level_of(l.hypot(r))),
                        pan: pans.get(&table).copied().unwrap_or(pan_of(l, r)),
                    }
                };
            }
            for (p, strip) in PLAYBACK.iter().enumerate() {
                let c = strip.first;
                let (l, r) = sides(ol, or, c, regs::Source::Playback);
                m.playback[submix][p] = Send { level: level_of(l.max(r)), pan: balance_of(l, r) };
            }
        }
        for (i, strip) in INPUTS.iter().enumerate() {
            let reg = channel_reg(Port::Input(strip.first), offset::MUTE_OR_FADER);
            m.input_mute[i] = channel.get(&reg).is_some_and(|&v| v != 0);
        }
        // the image from the loudest submix a strip is in: signs for phase, the crossed share
        // for a stereo strip's width
        let image = |strip: &Strip, source: fn(u8) -> regs::Source| {
            let c = strip.first;
            let loudest = OUTPUTS.iter().max_by(|a, b| {
                let total = |o: &Strip| g(o.first, source(c)) + g(o.first + 1, source(c)) + g(o.first, source(c + 1));
                total(a).total_cmp(&total(b))
            });
            let Some(out) = loudest else { return Image::PLAIN };
            let (ol, or) = (out.first, out.first + 1);
            if !strip.stereo {
                let l = signed(ol, source(c));
                let v = if l != 0.0 { l } else { signed(or, source(c)) };
                return Image { phase: [v < 0.0, false], width: 100 };
            }
            let (straight, crossed) = (signed(ol, source(c)), signed(ol, source(c + 1)));
            let (r_straight, r_crossed) = (signed(or, source(c + 1)), signed(or, source(c)));
            let total = straight.abs() + crossed.abs();
            if total == 0.0 {
                return Image::PLAIN;
            }
            let width = ((straight.abs() - crossed.abs()) / total * 100.0).round() as i16;
            let left = if straight != 0.0 { straight } else { r_crossed };
            let right = if r_straight != 0.0 { r_straight } else { crossed };
            Image { phase: [left < 0.0, right < 0.0], width }
        };
        for (i, strip) in INPUTS.iter().enumerate() {
            m.input_image[i] = image(strip, regs::Source::Input);
            // (an input's phase register says it even where it's in no submix)
            for (k, c) in strip.channels().enumerate() {
                if let Some(&v) = channel.get(&channel_reg(Port::Input(c), offset::INPUT_PHASE)) {
                    m.input_image[i].phase[k] = v != 0;
                }
            }
        }
        for (p, strip) in PLAYBACK.iter().enumerate() {
            m.playback_image[p] = image(strip, regs::Source::Playback);
        }
        for (settings, (strips, port)) in [&mut m.input_settings[..], &mut m.output_settings[..]]
            .into_iter()
            .zip([(&INPUTS[..], Port::Input as fn(u8) -> Port), (&OUTPUTS[..], Port::Output)])
        {
            for (s, strip) in settings.iter_mut().zip(strips) {
                for (off, _) in Settings::TOTALMIX.registers() {
                    if let Some(&v) = channel.get(&channel_reg(port(strip.first), off)) {
                        s.set_register(off, v);
                    }
                }
            }
        }
        for (i, p) in m.preamps.iter_mut().enumerate() {
            let c = i as u8;
            if let Some(&v) = channel.get(&channel_reg(Port::Input(c), offset::GAIN_REFLEVEL_OR_PHASE)) {
                p.gain = Some((v as i16).clamp(0, Preamp::max_gain(i)));
            }
            if Preamp::has_hi_z(i) {
                p.hi_z = channel.get(&channel_reg(Port::Input(c), offset::HI_Z)).is_some_and(|&v| v != 0);
            }
            p.autoset = channel.get(&channel_reg(Port::Input(c), offset::AUTOSET)).is_some_and(|&v| v != 0);
        }
        for (o, strip) in OUTPUTS.iter().enumerate() {
            let port = Port::Output(strip.first);
            m.outputs[o] = Output {
                level: channel.get(&channel_reg(port, offset::MUTE_OR_FADER)).map(|&v| (v as i16).max(LEVEL_OFF)),
                mute: channel.get(&channel_reg(port, offset::OUTPUT_MUTE)).is_some_and(|&v| v != 0),
            };
        }
        m
    }

    pub fn send(&self, submix: usize, source: Source) -> Send {
        match source {
            Source::Input(i) => self.inputs[submix][i],
            Source::Playback(p) => self.playback[submix][p],
        }
    }

    pub fn muted(&self, source: Source) -> bool {
        match source {
            Source::Input(i) => self.input_mute[i],
            Source::Playback(p) => self.playback_mute[p],
        }
    }

    /// Sets a strip's send in a submix and returns the words that put it into the unit.
    pub fn set_send(&mut self, submix: usize, source: Source, send: Send) -> Vec<Word> {
        let send = send.clamped();
        match source {
            Source::Input(i) => self.inputs[submix][i] = send,
            Source::Playback(p) => self.playback[submix][p] = send,
        }
        self.cross_words(submix, source)
    }

    /// Mutes a strip in every submix (its sends are kept): an input's mute register, and its
    /// matrix entries at zero, as TotalMix does.
    pub fn set_mute(&mut self, source: Source, mute: bool) -> Vec<Word> {
        let mut words = Vec::new();
        match source {
            Source::Input(i) => {
                self.input_mute[i] = mute;
                for c in INPUTS[i].channels() {
                    words.push(Word::new(channel_reg(Port::Input(c), offset::MUTE_OR_FADER), mute as u16));
                }
            }
            Source::Playback(p) => self.playback_mute[p] = mute,
        }
        for submix in 0..OUTPUTS.len() {
            words.extend(self.cross_words(submix, source));
        }
        words
    }

    pub fn image(&self, source: Source) -> Image {
        match source {
            Source::Input(i) => self.input_image[i],
            Source::Playback(p) => self.playback_image[p],
        }
    }

    /// A strip's phase and width, in every submix's matrix; an input's phase also in its channel
    /// register `0x06`.
    pub fn set_image(&mut self, source: Source, image: Image) -> Vec<Word> {
        let strip = source.strip();
        let image = Image {
            phase: [image.phase[0], image.phase[1] && strip.stereo],
            width: if strip.stereo { image.width.clamp(-100, 100) } else { 100 },
        };
        let mut words = Vec::new();
        match source {
            Source::Input(i) => {
                self.input_image[i] = image;
                for (k, c) in strip.channels().enumerate() {
                    words.push(Word::new(channel_reg(Port::Input(c), offset::INPUT_PHASE), image.phase[k] as u16));
                }
            }
            Source::Playback(p) => self.playback_image[p] = image,
        }
        for submix in 0..OUTPUTS.len() {
            words.extend(self.matrix_words(submix, source));
        }
        words
    }

    /// An output strip's level (tenths of a dB), on both its channels.
    pub fn set_output_level(&mut self, output: usize, level: i16) -> Vec<Word> {
        let level = level.clamp(LEVEL_OFF, LEVEL_MAX);
        self.outputs[output].level = Some(level);
        output_words(output, offset::MUTE_OR_FADER, level as u16)
    }

    pub fn set_output_mute(&mut self, output: usize, mute: bool) -> Vec<Word> {
        self.outputs[output].mute = mute;
        output_words(output, offset::OUTPUT_MUTE, mute as u16)
    }

    /// An input strip's own processing; returns the words for what changed, on both its channels.
    pub fn set_input_settings(&mut self, input: usize, settings: Settings) -> Vec<Word> {
        let new = settings.clamped();
        let words = settings_words(INPUTS[input], Port::Input, &self.input_settings[input], &new);
        self.input_settings[input] = new;
        words
    }

    /// An output strip's own processing; returns the words for what changed.
    pub fn set_output_settings(&mut self, output: usize, settings: Settings) -> Vec<Word> {
        let new = settings.clamped();
        let words = settings_words(OUTPUTS[output], Port::Output, &self.output_settings[output], &new);
        self.output_settings[output] = new;
        words
    }

    /// A front input's preamp (input strips 0–3); returns the words for what changed. Gain is
    /// clamped to the input's range, Hi-Z kept off where there is none.
    pub fn set_preamp(&mut self, input: usize, preamp: Preamp) -> Vec<Word> {
        let new = Preamp {
            gain: preamp.gain.map(|g| g.clamp(0, Preamp::max_gain(input))),
            hi_z: preamp.hi_z && Preamp::has_hi_z(input),
            autoset: preamp.autoset,
        };
        let old = preamp_registers(input, &self.preamps[input]);
        self.preamps[input] = new;
        preamp_registers(input, &new)
            .into_iter()
            .filter(|r| !old.contains(r))
            .map(|(addr, v)| Word::new(addr, v))
            .collect()
    }

    /// Every word of the state, as one push (TotalMix sends its whole state so after power-on),
    /// except what the unit keeps itself: the output levels and the preamps' gains. Mutes go
    /// first, then the matrix entries at zero, so that a push doesn't pass through louder mixes
    /// than its start and end.
    pub fn words(&self) -> Vec<Word> {
        let mut words = self.all_words();
        words.retain(|w| !self.unit_owned(*w));
        let rank = |w: &Word| {
            let off = (w.addr & 0xff) as u8;
            let channel_mute = w.addr < 0x2400
                && ((w.addr >> 8 < 0x12 && off == offset::MUTE_OR_FADER) || (w.addr >> 8 >= 0x12 && off == offset::OUTPUT_MUTE));
            match () {
                _ if channel_mute => 0,
                _ if is_matrix(w.addr) && w.value == 0 => 1,
                _ if is_matrix(w.addr) => 2,
                _ => 3,
            }
        };
        words.sort_by_key(rank);
        words
    }

    /// Output level and preamp gain registers: the unit's own.
    fn unit_owned(&self, w: Word) -> bool {
        let (channel, off) = (w.addr >> 8, (w.addr & 0xff) as u8);
        ((0x12..0x24).contains(&channel) && off == offset::MUTE_OR_FADER)
            || ((channel as usize) < PREAMPS && off == offset::GAIN_REFLEVEL_OR_PHASE)
    }

    /// The same mix with what the unit keeps itself unknown again: its output levels and the
    /// preamps' gains, which the front knob and encoder may have changed (after loading a mix
    /// saved earlier).
    pub fn without_unit_levels(mut self) -> Mixer {
        for o in &mut self.outputs {
            o.level = None;
        }
        for p in &mut self.preamps {
            p.gain = None;
        }
        self
    }

    /// Every value in the range the hardware takes (after loading a saved mix).
    pub fn clamped(mut self) -> Mixer {
        let rows = self.inputs.iter_mut().map(|r| &mut r[..]).chain(self.playback.iter_mut().map(|r| &mut r[..]));
        for send in rows.flatten() {
            *send = send.clamped();
        }
        for o in &mut self.outputs {
            o.level = o.level.map(|l| l.clamp(LEVEL_OFF, LEVEL_MAX));
        }
        for s in self.input_settings.iter_mut().chain(&mut self.output_settings) {
            *s = s.clamped();
        }
        for (i, p) in self.preamps.iter_mut().enumerate() {
            p.gain = p.gain.map(|g| g.clamp(0, Preamp::max_gain(i)));
            p.hi_z &= Preamp::has_hi_z(i);
        }
        for (image, strip) in self.input_image.iter_mut().zip(&INPUTS).chain(self.playback_image.iter_mut().zip(&PLAYBACK)) {
            image.width = if strip.stereo { image.width.clamp(-100, 100) } else { 100 };
            image.phase[1] &= strip.stereo;
        }
        self
    }

    /// Every word of the state, the unit's own levels and gains among them where known.
    fn all_words(&self) -> Vec<Word> {
        let mut words = Vec::new();
        for (i, strip) in INPUTS.iter().enumerate() {
            for (k, c) in strip.channels().enumerate() {
                let mute = self.input_mute[i] as u16;
                words.push(Word::new(channel_reg(Port::Input(c), offset::MUTE_OR_FADER), mute));
                let phase = self.input_image[i].phase[k] as u16;
                words.push(Word::new(channel_reg(Port::Input(c), offset::INPUT_PHASE), phase));
            }
        }
        for submix in 0..OUTPUTS.len() {
            for i in 0..INPUTS.len() {
                words.extend(self.cross_words(submix, Source::Input(i)));
            }
            for p in 0..PLAYBACK.len() {
                words.extend(self.cross_words(submix, Source::Playback(p)));
            }
        }
        for (o, out) in self.outputs.iter().enumerate() {
            if let Some(level) = out.level {
                words.extend(output_words(o, offset::MUTE_OR_FADER, level as u16));
            }
            words.extend(output_words(o, offset::OUTPUT_MUTE, out.mute as u16));
        }
        for (i, strip) in INPUTS.iter().enumerate() {
            words.extend(all_settings_words(*strip, Port::Input, &self.input_settings[i]));
        }
        for (i, p) in self.preamps.iter().enumerate() {
            words.extend(preamp_registers(i, p).into_iter().map(|(addr, v)| Word::new(addr, v)));
        }
        for (o, strip) in OUTPUTS.iter().enumerate() {
            words.extend(all_settings_words(*strip, Port::Output, &self.output_settings[o]));
        }
        words
    }

    /// The matrix entries of a strip in a submix: a mono strip to both sides by its pan, a stereo
    /// one side to side by its balance and width (cross terms at zero at full width); phase as
    /// the gains' signs.
    fn matrix_words(&self, submix: usize, source: Source) -> Vec<Word> {
        let send = self.send(submix, source);
        let strip = source.strip();
        let out = OUTPUTS[submix];
        let (ol, or) = (out.first, out.first + 1);
        let g = if self.muted(source) { 0.0 } else { gain(send.level) };
        let image = self.image(source);
        let c = strip.first;
        if strip.stereo {
            // each output side takes its own channel's straight share and the other's crossed one
            let (l, r) = balance_sides(send.pan);
            let (a, b) = image.spread();
            let (sl, sr) = (image.sign(0), image.sign(1));
            vec![
                matrix_word(ol, source.matrix(c), g * l * a * sl),
                matrix_word(or, source.matrix(c + 1), g * r * a * sr),
                matrix_word(ol, source.matrix(c + 1), g * l * b * sr),
                matrix_word(or, source.matrix(c), g * r * b * sl),
            ]
        } else {
            let (l, r) = pan_sides(send.pan);
            let s = image.sign(0);
            vec![matrix_word(ol, source.matrix(c), g * l * s), matrix_word(or, source.matrix(c), g * r * s)]
        }
    }

    /// The matrix entries, and for an input its fader table entries: for each of the submix's
    /// output channels and each of the input's, the level that channel gets and its pan. As
    /// TotalMix writes them: a stereo input's channels at pan −100 and +100, each at the level
    /// its side of the balance leaves it (R50: the left 6 dB down); a muted input at −∞.
    fn cross_words(&self, submix: usize, source: Source) -> Vec<Word> {
        let mut words = self.matrix_words(submix, source);
        if let Source::Input(_) = source {
            let send = self.send(submix, source);
            let strip = source.strip();
            let (l, r) = balance_sides(send.pan);
            for o in OUTPUTS[submix].channels() {
                for c in strip.channels() {
                    let (pan, side) = if !strip.stereo {
                        (send.pan, 1.0)
                    } else if c == strip.first {
                        (-100, l)
                    } else {
                        (100, r)
                    };
                    let level = if self.muted(source) {
                        LEVEL_OFF
                    } else if side >= 1.0 {
                        send.level
                    } else {
                        level_of(gain(send.level) * side)
                    };
                    let reg = fader_table_reg(o, c);
                    words.push(Word::new(reg, FaderTableField::Level(level).encode()));
                    words.push(Word::new(reg, FaderTableField::Pan(pan).encode()));
                }
            }
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(words: &[Word], addr: u16) -> Option<u16> {
        words.iter().rev().find(|w| w.addr == addr).map(|w| w.value)
    }

    /// Both fields of a fader table entry in `words`, latest first.
    fn table(words: &[Word], addr: u16) -> (Option<i16>, Option<i16>) {
        let fields = words.iter().rev().filter(|w| w.addr == addr).map(|w| FaderTableField::decode(w.value));
        let mut level = None;
        let mut pan = None;
        for f in fields {
            match f {
                FaderTableField::Level(t) => level = level.or(Some(t)),
                FaderTableField::Pan(p) => pan = pan.or(Some(p)),
            }
        }
        (level, pan)
    }

    fn db(value: u16) -> f64 {
        MatrixGain(value).db()
    }

    #[test]
    fn totalmix_state_is_the_default_routing() {
        let m = Mixer::totalmix();
        for p in 0..PLAYBACK.len() {
            assert_eq!(m.playback[p][p], Send { level: 0, pan: 0 }, "playback {p} on its own output");
            assert_eq!(m.playback[3][p], Send { level: 0, pan: 0 }, "playback {p} on PH 7/8");
        }
        assert_eq!(m.playback[0][1], Send::OFF);
        assert_eq!(m.inputs[0][5], Send { level: -106, pan: 0 }, "AN 7/8 -> AN 1/2 at -10.6 dB");
        assert_eq!(m.inputs[1][5], Send::OFF);
        assert_eq!(m.inputs[0][0], Send::OFF);
        assert!(m.outputs.iter().all(|o| *o == Output { level: None, mute: false }));
        assert!(!m.input_mute.iter().any(|&x| x));
    }

    #[test]
    fn totalmix_state_pushed_again_gives_the_same_words() {
        let push = totalmix_state();
        let ours = Mixer::totalmix().words();
        for w in &push {
            // (the FX send bus, matrix outputs 30/31, isn't modelled yet: the push has it all at
            // zero, and ours leaves it alone)
            let fx = is_matrix(w.addr) && (w.addr - 0x4000) / 0x40 >= regs::FX_BUS[0] as u16;
            if is_matrix(w.addr) && !fx {
                let v = value(&ours, w.addr).unwrap_or_else(|| panic!("{:04x} not written", w.addr));
                if w.value == 0 {
                    assert_eq!(v, 0, "{:04x}", w.addr);
                } else {
                    assert!((db(v) - db(w.value)).abs() < 0.06, "{:04x}: {v:04x} for {:04x}", w.addr, w.value);
                }
            } else if is_fader_table(w.addr) {
                let (level, pan) = table(&push, w.addr);
                let (our_level, our_pan) = table(&ours, w.addr);
                assert_eq!(level, our_level, "{:04x} level", w.addr);
                // (the push leaves out the pan of stereo strips at −∞)
                if pan.is_some() {
                    assert_eq!(pan, our_pan, "{:04x} pan", w.addr);
                }
            } else if (0x1200..0x2400).contains(&w.addr) {
                assert_eq!(value(&ours, w.addr), Some(w.value), "{:04x}", w.addr);
            }
        }
    }

    #[test]
    fn fader_moves_write_what_totalmix_wrote() {
        // re/maps/matrix.md, submix AN 1/2 at -12.1 dB: Mic 1 (mono, centre), AN 5/6 (stereo),
        // and playback AN 1/2 at 0 dB
        let mut m = Mixer::silent();
        let w = m.set_send(0, Source::Input(0), Send { level: -121, pan: 0 });
        for addr in [0x4000, 0x4040] {
            assert!((db(value(&w, addr).unwrap()) - db(0x1668)).abs() < 0.06);
        }
        assert_eq!(table(&w, 0x12e0), (Some(-121), Some(0)));
        assert_eq!(table(&w, 0x13e0), (Some(-121), Some(0)));

        let w = m.set_send(0, Source::Input(4), Send { level: -121, pan: 0 });
        for addr in [0x4004, 0x4045] {
            assert!((db(value(&w, addr).unwrap()) - db(0x1fb1)).abs() < 0.06);
        }
        assert_eq!((value(&w, 0x4005), value(&w, 0x4044)), (Some(0), Some(0)));
        assert_eq!(table(&w, 0x12e4), (Some(-121), Some(-100)));
        assert_eq!(table(&w, 0x13e5), (Some(-121), Some(100)));

        let w = m.set_send(0, Source::Playback(0), Send { level: 0, pan: 0 });
        assert_eq!((value(&w, 0x4020), value(&w, 0x4061)), (Some(0x9000), Some(0x9000)));
        assert_eq!((value(&w, 0x4021), value(&w, 0x4060)), (Some(0), Some(0)));
        assert!(w.iter().all(|w| is_matrix(w.addr)), "playback has no fader table");
    }

    #[test]
    fn pan_and_balance_curves() {
        let (l, r) = pan_sides(0);
        assert!((l - r).abs() < 1e-12 && (20.0 * l.log10() + 3.0103).abs() < 1e-3);
        assert_eq!(pan_sides(-100), (1.0, 0.0));
        assert!(pan_sides(100).0.abs() < 1e-12);
        assert_eq!(balance_sides(0), (1.0, 1.0));
        assert_eq!(balance_sides(50), (0.5, 1.0), "linear, as TotalMix's");
        assert_eq!(balance_sides(-100), (1.0, 0.0));
        for p in [-100, -60, -1, 0, 1, 33, 100] {
            let (l, r) = pan_sides(p);
            assert_eq!(pan_of(l * 0.3, r * 0.3), p);
            let (l, r) = balance_sides(p);
            assert_eq!(balance_of(l * 0.3, r * 0.3), p);
        }
    }

    #[test]
    fn panned_and_balanced_sends_come_back_from_their_words() {
        let mut m = Mixer::silent();
        m.set_send(2, Source::Input(1), Send { level: -60, pan: 40 });
        m.set_send(2, Source::Playback(4), Send { level: 30, pan: -70 });
        let back = Mixer::from_words(m.words());
        assert_eq!(back.inputs[2][1], Send { level: -60, pan: 40 });
        assert_eq!(back.playback[2][4], Send { level: 30, pan: -70 });
        assert_eq!(back, m);
    }

    #[test]
    fn mute_zeroes_the_matrix_and_keeps_the_sends() {
        let mut m = Mixer::totalmix();
        let w = m.set_mute(Source::Input(5), true);
        assert_eq!((value(&w, 0x0600), value(&w, 0x0700)), (Some(1), Some(1)));
        assert_eq!(value(&w, 0x4006), Some(0));
        assert_eq!(m.inputs[0][5].level, -106);
        let w = m.set_mute(Source::Input(5), false);
        assert!((db(value(&w, 0x4006).unwrap()) + 10.6).abs() < 0.06);
        let w = m.set_mute(Source::Playback(0), true);
        assert_eq!(value(&w, 0x4020), Some(0));
        assert!(!w.iter().any(|w| w.addr < 0x4000), "playback has no mute register");
    }

    #[test]
    fn outputs_write_both_channels() {
        let mut m = Mixer::silent();
        let w = m.set_output_level(3, -200);
        assert_eq!(w, vec![Word::new(0x1800, (-200i16) as u16), Word::new(0x1900, (-200i16) as u16)]);
        let w = m.set_output_mute(0, true);
        assert_eq!(w, vec![Word::new(0x1202, 1), Word::new(0x1302, 1)]);
        // the unit keeps its output levels: a whole push leaves them out
        assert!(!m.words().iter().any(|w| w.addr == 0x1800 || w.addr == 0x1900));
        assert_eq!(Mixer::from_words(m.words()).outputs[3].level, None);
    }

    #[test]
    fn only_the_digital_outputs_words_are_digital_only() {
        let mut m = Mixer::totalmix();
        for submix in DIGITAL_OUTPUTS {
            for source in [Source::Input(0), Source::Input(4), Source::Playback(1)] {
                assert!(m.set_send(submix, source, Send { level: 0, pan: 30 }).into_iter().all(digital_only));
            }
            assert!(m.set_output_level(submix, -100).into_iter().all(digital_only));
            assert!(m.set_output_mute(submix, true).into_iter().all(digital_only));
        }
        for submix in 0..DIGITAL_OUTPUTS.start {
            for source in [Source::Input(0), Source::Input(4), Source::Playback(1)] {
                assert!(m.set_send(submix, source, Send { level: 0, pan: 0 }).into_iter().all(|w| !digital_only(w)));
            }
            assert!(m.set_output_level(submix, 0).into_iter().all(|w| !digital_only(w)));
            assert!(m.set_output_mute(submix, false).into_iter().all(|w| !digital_only(w)));
        }
        // input settings (gain, the unmapped offsets phantom power is among), mutes, the FX bus,
        // globals, an analog output's fader table
        for addr in [0x0000, 0x0007, 0x0001, 0x0600, 0x1200, 0x1902, 0x47a0, 0x4780, 0x3f9e, 0x3d01, 0x19e0] {
            assert!(!digital_only(Word::new(addr, 0)), "{addr:04x}");
        }
        // a mute reaches every submix, the analog ones too
        assert!(!m.set_mute(Source::Input(0), true).into_iter().all(digital_only));
    }

    #[test]
    fn settings_write_what_totalmix_wrote() {
        // re/maps/channel-settings.md, Mic 1 and the stereo strips
        let mut m = Mixer::silent();
        let mut s = Settings::TOTALMIX;
        s.eq.bands[0].gain = -200;
        s.eq.bands[2].kind = 2;
        s.low_cut.slope = 3;
        s.dynamics.comp_threshold = -600;
        s.dynamics.exp_threshold = -990;
        s.dynamics.release = 999;
        s.auto_level.rise = 99;
        let w = m.set_input_settings(0, s);
        let expect = [(0x0042, 0xff38), (0x0048, 0x0002), (0x0022, 0x0003), (0x0064, 0xfda8), (0x0066, 0xfc22),
            (0x0063, 0x03e7), (0x0083, 0x0063)];
        assert_eq!(w.len(), expect.len(), "only what changed");
        for (addr, v) in expect {
            assert_eq!(value(&w, addr), Some(v), "{addr:04x}");
        }
        // a pair writes both channels; an output strip its output channels
        let mut on = Settings::TOTALMIX;
        on.eq.on = true;
        assert_eq!(m.set_input_settings(4, on), vec![Word::new(0x0440, 1), Word::new(0x0540, 1)]);
        assert_eq!(m.set_output_settings(0, on), vec![Word::new(0x1240, 1), Word::new(0x1340, 1)]);
        assert!(m.set_output_settings(0, on).is_empty(), "nothing changed");
        // and come back from a push
        let back = Mixer::from_words(m.words());
        assert_eq!((back.input_settings, back.output_settings), (m.input_settings, m.output_settings));
    }

    #[test]
    fn the_digital_outputs_processing_is_digital_only() {
        let mut m = Mixer::silent();
        let mut s = Settings::TOTALMIX;
        s.dynamics.on = true;
        s.low_cut.freq = 300;
        assert!(m.set_output_settings(4, s).into_iter().all(digital_only), "SPDIF");
        assert!(m.set_output_settings(8, s).into_iter().all(digital_only), "ADAT 7/8");
        assert!(m.set_output_settings(3, s).into_iter().all(|w| !digital_only(w)), "PH 7/8");
        assert!(m.set_input_settings(7, s).into_iter().all(|w| !digital_only(w)), "an input reaches every output");
        // still nothing else on a digital output's channel: its phase, reference level, FX return
        for off in [0x01, 0x03, 0x07, 0x08] {
            assert!(!digital_only(Word::new(0x1a00 | off, 0)), "{off:02x}");
        }
    }

    #[test]
    fn preamps_write_what_totalmix_wrote() {
        // re/maps/audible-*.md: Mic 1 gain 65 dB, Mic 2 49 dB, AN 3 12 dB, AN 3 Hi-Z, AutoSet
        let mut m = Mixer::silent();
        let gain = |g| Preamp { gain: Some(g), ..Preamp::FOUND };
        assert_eq!(m.set_preamp(0, gain(650)), vec![Word::new(0x0007, 0x028a)]);
        assert_eq!(m.set_preamp(1, gain(490)), vec![Word::new(0x0107, 0x01ea)]);
        assert_eq!(m.set_preamp(2, gain(120)), vec![Word::new(0x0207, 0x0078)]);
        assert_eq!(m.set_preamp(2, Preamp { hi_z: true, ..m.preamps[2] }), vec![Word::new(0x0209, 1)]);
        assert_eq!(m.set_preamp(3, Preamp { autoset: true, ..Preamp::FOUND }), vec![Word::new(0x030a, 1)]);
        // AN 3/4 stop at 16 dB, the mics have no Hi-Z
        assert_eq!(m.set_preamp(3, gain(400)), vec![Word::new(0x0307, 160), Word::new(0x030a, 0)]);
        assert!(m.set_preamp(0, Preamp { hi_z: true, ..m.preamps[0] }).is_empty());
        // none of it is digital-only, and a push carries it back
        assert!(m.words().into_iter().filter(|w| w.addr < 0x0400 && (w.addr & 0xff) >= 0x07 && (w.addr & 0xff) <= 0x0a).all(|w| !digital_only(w)));
        // a whole push leaves the gains out (the unit keeps them), Hi-Z and AutoSet in
        let back = Mixer::from_words(m.words()).preamps;
        assert_eq!(back.map(|p| (p.gain, p.hi_z, p.autoset)), m.preamps.map(|p| (None, p.hi_z, p.autoset)));
    }

    #[test]
    fn width_and_phase_write_the_matrix_as_totalmix_did() {
        let mut m = Mixer::silent();
        m.set_send(0, Source::Playback(0), Send { level: 0, pan: 0 });
        // width 0: 0.5 to all four cross-points (captured)
        let w = m.set_image(Source::Playback(0), Image { phase: [false; 2], width: 0 });
        for addr in [0x4020, 0x4021, 0x4060, 0x4061] {
            assert_eq!(value(&w, addr), Some(0x8800), "{addr:04x}");
        }
        // phase on the left: its straight and crossed gains negative (-1.0 = 0xf000)
        let w = m.set_image(Source::Playback(0), Image { phase: [true, false], width: 100 });
        assert_eq!((value(&w, 0x4020), value(&w, 0x4061)), (Some(0xf000), Some(0x9000)));
        assert_eq!((value(&w, 0x4021), value(&w, 0x4060)), (Some(0), Some(0)));
        // an input's phase is also its register 0x06; a mono strip has no right side or width
        let w = m.set_image(Source::Input(0), Image { phase: [true, true], width: 0 });
        assert_eq!(value(&w, 0x0006), Some(1));
        assert_eq!(m.input_image[0], Image { phase: [true, false], width: 100 });
        // and they come back from the words
        m.set_send(2, Source::Input(4), Send { level: -60, pan: 30 });
        m.set_image(Source::Input(4), Image { phase: [false, true], width: 40 });
        let back = Mixer::from_words(m.words());
        assert_eq!((back.input_image, back.playback_image), (m.input_image, m.playback_image));
        assert_eq!(back.inputs[2][4], m.inputs[2][4], "the send under the image");
    }

    #[test]
    fn balance_and_mute_write_what_totalmix_wrote() {
        // re/maps/audible-rest.md: AN 7/8 into AN 1/2 at -10.6 dB, balance R50: the left gain
        // halved (258f -> 12c7) and its fader table entry 6 dB down (f960 -> f590)
        let mut m = Mixer::totalmix();
        let w = m.set_send(0, Source::Input(5), Send { level: -106, pan: 50 });
        assert!((db(value(&w, 0x4006).unwrap()) - db(0x12c7)).abs() < 0.06);
        assert!((db(value(&w, 0x4047).unwrap()) - db(0x258f)).abs() < 0.06, "the right side as it was");
        let (level, pan) = table(&w, 0x12e6);
        assert!((level.unwrap() + 167).abs() <= 1 && pan == Some(-100), "{level:?}");
        assert_eq!(table(&w, 0x12e7), (Some(-106), Some(100)));
        // and the send comes back from those words
        assert_eq!(Mixer::from_words(m.words()).inputs[0][5], Send { level: -106, pan: 50 });
        // a muted input's table entries at -inf, as TotalMix writes d760; back on unmuting
        let w = m.set_mute(Source::Input(5), true);
        assert_eq!(table(&w, 0x12e7).0, Some(LEVEL_OFF));
        let w = m.set_mute(Source::Input(5), false);
        assert_eq!(table(&w, 0x12e7).0, Some(-106));
    }

    #[test]
    fn a_whole_push_mutes_and_silences_first_and_leaves_the_units_levels_out() {
        let mut m = Mixer::totalmix();
        m.set_output_level(0, -200);
        m.set_preamp(0, Preamp { gain: Some(300), ..Preamp::FOUND });
        let w = m.words();
        assert!(!w.iter().any(|w| m.unit_owned(*w)), "no output levels or preamp gains");
        let first_nonzero_matrix = w.iter().position(|w| is_matrix(w.addr) && w.value != 0).unwrap();
        let last_zero_matrix = w.iter().rposition(|w| is_matrix(w.addr) && w.value == 0).unwrap();
        let last_mute = w.iter().rposition(|w| w.addr & 0xff == 0x02 && (0x12..0x24).contains(&(w.addr >> 8))).unwrap();
        assert!(last_mute < last_zero_matrix && last_zero_matrix < first_nonzero_matrix);
    }

    #[test]
    fn a_saved_mix_is_clamped_and_forgets_the_units_levels() {
        let mut m = Mixer::totalmix();
        m.outputs[0].level = Some(3000);
        m.preamps[0].gain = Some(2000);
        m.preamps[0].hi_z = true;
        m.inputs[1][2].level = 900;
        m.input_settings[3].eq.bands[0].freq = 50000;
        m.playback_image[2].width = 400;
        m.input_image[0].width = 0;
        let c = m.clone().clamped();
        assert_eq!((c.outputs[0].level, c.preamps[0].gain, c.preamps[0].hi_z), (Some(LEVEL_MAX), Some(650), false));
        assert_eq!((c.inputs[1][2].level, c.input_settings[3].eq.bands[0].freq), (LEVEL_MAX, 20000));
        assert_eq!((c.playback_image[2].width, c.input_image[0].width), (100, 100));
        let f = c.without_unit_levels();
        assert_eq!((f.outputs[0].level, f.preamps[0].gain), (None, None));
    }

    #[test]
    fn state_saved_before_the_settings_loads_with_totalmix_settings() {
        let mut json: serde_json::Value = serde_json::to_value(Mixer::totalmix()).unwrap();
        let obj = json.as_object_mut().unwrap();
        obj.remove("input_settings");
        obj.remove("output_settings");
        obj.remove("preamps");
        obj.remove("input_image");
        obj.remove("playback_image");
        let old: Mixer = serde_json::from_value(json).unwrap();
        assert_eq!(old, Mixer::totalmix());
    }

    #[test]
    fn saves_and_loads() {
        let mut m = Mixer::totalmix();
        m.set_send(4, Source::Input(2), Send { level: -35, pan: -20 });
        let json = serde_json::to_string(&m).unwrap();
        assert_eq!(serde_json::from_str::<Mixer>(&json).unwrap(), m);
    }
}
