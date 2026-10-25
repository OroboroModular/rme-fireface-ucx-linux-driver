//! The card with a strip's own processing (TotalMix's channel settings, `ucx_mixer::settings`):
//! a three-band EQ, low cut, dynamics and auto-level, in the family's knobs, pill toggles and
//! step pickers. Knobs turn in the parameters' own units (frequencies on a log scale) and reset
//! to the value the probes found (`Settings::TOTALMIX`).
//!
//! An input's processing reaches every output, so it's held back unless the analog outputs may
//! be reached; a digital output's goes out once sending is on (`MixerUi::apply`).

use std::cell::Cell;

use oroboro_egui::egui::{pos2, vec2, Id, Pos2, Rect, Ui, Vec2};
use oroboro_egui::theme::*;
use oroboro_egui::widgets::{self as look, ButtonKind, HostParam};
use ucx_mixer::settings::Settings;
use ucx_mixer::{Image, Preamp, Source, INPUTS, OUTPUTS, PLAYBACK, PREAMPS};

use crate::view::MixerUi;

/// Whose processing the card shows. Playback strips have none in hardware: their card has only
/// the stereo image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Edit {
    Input(usize),
    Playback(usize),
    Output(usize),
}

impl Edit {
    pub fn name(self) -> &'static str {
        match self {
            Edit::Input(i) => INPUTS[i].name,
            Edit::Playback(p) => PLAYBACK[p].name,
            Edit::Output(o) => OUTPUTS[o].name,
        }
    }

    /// The source strip whose image the card shows, if it has one.
    fn source(self) -> Option<Source> {
        match self {
            Edit::Input(i) => Some(Source::Input(i)),
            Edit::Playback(p) => Some(Source::Playback(p)),
            Edit::Output(_) => None,
        }
    }
}

/// The card, and a playback strip's (only its image).
pub const CARD: Vec2 = vec2(924.0, 474.0);
pub const PLAYBACK_CARD: Vec2 = vec2(480.0, 240.0);
/// Between controls, and between groups.
const GAP: f32 = 12.0;
const GROUP_GAP: f32 = 28.0;
/// A row: its group title, then the knobs' cells.
const TITLE: f32 = 18.0;
const ROW: f32 = TITLE + 86.0 + 16.0;

/// A choice among names, for the family's step picker: shows `index`, a click leaves the new
/// one in `picked`.
struct Choice {
    key: String,
    index: usize,
    names: &'static [&'static str],
    picked: Cell<Option<usize>>,
}

impl Choice {
    fn new(key: String, index: usize, names: &'static [&'static str]) -> Choice {
        Choice { key, index: index.min(names.len() - 1), names, picked: Cell::new(None) }
    }

    fn last(&self) -> f32 {
        (self.names.len() - 1).max(1) as f32
    }

    fn index_at(&self, position: f32) -> usize {
        ((position * self.last()).round() as usize).min(self.names.len() - 1)
    }
}

impl HostParam for Choice {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn position(&self) -> f32 {
        self.index as f32 / self.last()
    }
    fn default_position(&self) -> f32 {
        0.0
    }
    fn text_at(&self, position: f32) -> String {
        self.names[self.index_at(position)].into()
    }
    fn step_from(&self, position: f32, up: bool) -> f32 {
        let i = self.index_at(position);
        let i = if up { (i + 1).min(self.names.len() - 1) } else { i.saturating_sub(1) };
        i as f32 / self.last()
    }
    fn begin(&self) {}
    fn set(&self, position: f32) {
        self.picked.set(Some(self.index_at(position)));
    }
    fn end(&self) {}
}

/// On/off, for the family's pill toggle, labelled `label`.
fn labelled(ui: &mut Ui, rect: Rect, key: String, on: bool, label: &str) -> bool {
    let flag = crate::widgets::Flag::new(key, on);
    look::toggle(ui, rect, &flag, label);
    flag.clicked.get().unwrap_or(on)
}

/// A group's On toggle.
fn switch(ui: &mut Ui, rect: Rect, key: String, on: bool) -> bool {
    labelled(ui, rect, key, on, "On")
}

/// A frequency as the family writes it: "80 Hz", "1.20 kHz".
fn hz(f: f32) -> String {
    if f < 1000.0 {
        format!("{f:.0} Hz")
    } else {
        format!("{:.2} kHz", f / 1000.0)
    }
}

/// Lays out a row of controls left to right from `at`.
struct Row {
    x: f32,
    y: f32,
}

impl Row {
    fn cell(&mut self) -> Rect {
        let r = Rect::from_min_size(pos2(self.x, self.y), look::knob_cell());
        self.x += look::knob_cell().x;
        r
    }

    /// A toggle, centred on a knob's height.
    fn toggle(&mut self) -> Rect {
        let r = Rect::from_min_size(pos2(self.x, self.y + 14.0), vec2(64.0, 22.0));
        self.x += 64.0 + GAP;
        r
    }

    /// A step picker, `width` wide (the family's is `look::STEP`).
    fn step(&mut self, width: f32) -> Rect {
        let r = Rect::from_min_size(pos2(self.x, self.y + 10.0), vec2(width, look::STEP.y));
        self.x += width + GAP;
        r
    }

    fn gap(&mut self, w: f32) {
        self.x += w;
    }
}

/// A knob over `range` in the parameter's own units (`log` for frequencies), resetting to
/// `default`. Returns the new value when it turned.
#[allow(clippy::too_many_arguments)]
fn dial(
    ui: &mut Ui,
    cell: Rect,
    id: Id,
    value: f32,
    (min, max): (f32, f32),
    log: bool,
    default: f32,
    text: impl Fn(f32) -> String,
    label: &str,
    lit: bool,
) -> Option<f32> {
    let to = |v: f32| if log { (v / min).ln() / (max / min).ln() } else { (v - min) / (max - min) };
    let from = |p: f32| if log { min * (max / min).powf(p) } else { min + p * (max - min) };
    let turn = look::knob_at(ui, cell, id, to(value).clamp(0.0, 1.0), to(default), |p| text(from(p)), label, lit);
    turn.value.map(from)
}

/// Draws the card for `edit` over `area` and applies what changes on it.
pub fn card(ui: &mut Ui, s: &mut MixerUi, edit: Edit, area: Rect) {
    let ctx = ui.ctx().clone();
    look::overlay(&ctx, "ucx-processing", area, |ui| {
        let size = if let Edit::Playback(_) = edit { PLAYBACK_CARD } else { CARD };
        let rect = look::card_rect(ui, area, size, "ucx-processing");
        let body = look::card(ui, rect, &format!("{} · processing", edit.name()));
        let old = match edit {
            Edit::Input(i) => Some(s.mixer.input_settings[i]),
            Edit::Output(o) => Some(s.mixer.output_settings[o]),
            Edit::Playback(_) => None,
        };
        if let Some(old) = old {
            let new = controls(ui, edit, old, body.min);
            if new != old {
                match edit {
                    Edit::Input(i) => s.change_input_settings(i, new),
                    Edit::Output(o) => s.change_output_settings(o, new),
                    Edit::Playback(_) => {}
                }
            }
        }
        if let Some(source) = edit.source() {
            let at = match edit {
                Edit::Playback(_) => body.min,
                _ => body.min + vec2(IMAGE_X, 2.0 * ROW),
            };
            let old = s.mixer.image(source);
            let new = image(ui, edit, source, old, at);
            if new != old {
                s.change_image(source, new);
            }
        }
        if let Edit::Input(i) = edit {
            if i < PREAMPS {
                let old = s.mixer.preamps[i];
                let new = preamp(ui, i, old, body.min + vec2(PREAMP_X, ROW));
                if new != old {
                    s.change_preamp(i, new);
                }
            }
        }
        let painter = ui.painter();
        let note = match edit {
            Edit::Input(_) => "An input's processing reaches every output: it's sent with Analog outputs on.",
            Edit::Playback(_) => "The same in every submix (no EQ on playback).",
            Edit::Output(o) if ucx_mixer::DIGITAL_OUTPUTS.contains(&o) => {
                "A digital output's processing is sent with Send to unit on."
            }
            Edit::Output(_) => "An analog output's processing is sent with Analog outputs on.",
        };
        let at = pos2(body.left(), body.bottom() - 14.0);
        painter.text(at, oroboro_egui::egui::Align2::LEFT_CENTER, note, font(11.0), MUTED());
        let done = Rect::from_min_size(pos2(body.right() - 100.0, body.bottom() - 28.0), vec2(100.0, 28.0));
        let closed = look::button(ui, done, "Done", ButtonKind::Primary, false, "ucx-processing-done").clicked()
            || ui.input(|i| i.key_pressed(oroboro_egui::egui::Key::Escape));
        if closed {
            s.editing = None;
        }
    });
}

/// Where the preamp group sits in the second row, after low cut and auto-level, and the stereo
/// image in the third, after the dynamics.
const PREAMP_X: f32 = 580.0;
const IMAGE_X: f32 = 576.0;

/// A source strip's image from `top_left`: each channel's phase, and a stereo strip's width.
fn image(ui: &mut Ui, edit: Edit, source: Source, old: Image, top_left: Pos2) -> Image {
    let mut m = old;
    look::group_title(ui.painter(), top_left, "Stereo");
    let mut row = Row { x: top_left.x, y: top_left.y + TITLE };
    let stereo = source.strip().stereo;
    let toggle = |row: &mut Row| {
        let r = Rect::from_min_size(pos2(row.x, row.y + 14.0), vec2(90.0, 22.0));
        row.x += 90.0 + GAP;
        r
    };
    let (left, right) = if stereo { ("Phase L", "Phase R") } else { ("Phase", "") };
    m.phase[0] = labelled(ui, toggle(&mut row), format!("ucx-image-{edit:?}-l"), m.phase[0], left);
    if stereo {
        m.phase[1] = labelled(ui, toggle(&mut row), format!("ucx-image-{edit:?}-r"), m.phase[1], right);
        let text = |v: f32| match v.round() as i16 {
            100 => "Full".to_string(),
            0 => "Mono".to_string(),
            -100 => "Swapped".to_string(),
            w => format!("{w}"),
        };
        let id = Id::new(("ucx-image-width", edit));
        if let Some(v) = dial(ui, row.cell(), id, m.width as f32, (-100.0, 100.0), false, 100.0, text, "Width", true) {
            m.width = v.round() as i16;
        }
    }
    m
}

/// A front input's preamp from `top_left`: gain (unknown until set: the unit keeps it, and its
/// front encoder sets it too), Hi-Z on AN 3/4, AutoSet.
fn preamp(ui: &mut Ui, i: usize, old: Preamp, top_left: Pos2) -> Preamp {
    let mut p = old;
    look::group_title(ui.painter(), top_left, "Preamp");
    let mut row = Row { x: top_left.x, y: top_left.y + TITLE };
    let max = Preamp::max_gain(i) as f32 / 10.0;
    let known = p.gain.is_some();
    let gain = p.gain.unwrap_or(0) as f32 / 10.0;
    let text = |v: f32| if known { format!("{v:.0} dB") } else { "?".into() };
    let id = Id::new(("ucx-preamp", i));
    if let Some(v) = dial(ui, row.cell(), id, gain, (0.0, max), false, 0.0, text, "Gain", known) {
        // whole dB, as every captured gain was
        p.gain = Some((v.round() * 10.0) as i16);
    }
    row.gap(GAP);
    let toggle = |row: &mut Row| {
        let r = Rect::from_min_size(pos2(row.x, row.y + 14.0), vec2(90.0, 22.0));
        row.x += 90.0 + GAP;
        r
    };
    if Preamp::has_hi_z(i) {
        p.hi_z = labelled(ui, toggle(&mut row), format!("ucx-preamp-{i}-hiz"), p.hi_z, "Hi-Z");
    }
    p.autoset = labelled(ui, toggle(&mut row), format!("ucx-preamp-{i}-autoset"), p.autoset, "AutoSet");
    p
}

/// The controls from `top_left`, three rows: EQ; low cut and auto-level; dynamics.
fn controls(ui: &mut Ui, edit: Edit, old: Settings, top_left: Pos2) -> Settings {
    let mut s = old;
    let d = Settings::TOTALMIX;
    let id = |what: &str| Id::new(("ucx-processing", edit, what));
    let key = |what: &str| format!("ucx-processing-{edit:?}-{what}");
    let title = |ui: &Ui, x: f32, y: f32, t: &str| look::group_title(ui.painter(), pos2(x, y), t);
    let db = |v: f32| format!("{v:.1} dB");
    let x0 = top_left.x;

    // EQ: on, then each band's type (bands 1 and 3), gain, frequency and Q
    let y = top_left.y;
    title(ui, x0, y, "EQ");
    let mut row = Row { x: x0, y: y + TITLE };
    let lit = s.eq.on;
    s.eq.on = switch(ui, row.toggle(), key("eq"), s.eq.on);
    for b in 0..3 {
        title(ui, row.x, y, &format!("Band {}", b + 1));
        let names: &'static [&'static str] = match b {
            0 => &["Bell", "Shelf"],
            2 => &["Bell", "Shelf", "Low Pass"],
            _ => &[],
        };
        if !names.is_empty() {
            let choice = Choice::new(key(&format!("type{b}")), s.eq.bands[b].kind as usize, names);
            look::step_picker(ui, row.step(look::STEP.x), &choice);
            if let Some(k) = choice.picked.get() {
                s.eq.bands[b].kind = k as u8;
            }
        }
        let band = s.eq.bands[b];
        let def = d.eq.bands[b];
        if let Some(v) = dial(ui, row.cell(), id(&format!("gain{b}")), band.gain as f32 / 10.0, (-20.0, 20.0), false,
            def.gain as f32 / 10.0, db, "Gain", lit)
        {
            s.eq.bands[b].gain = (v * 10.0).round() as i16;
        }
        if let Some(v) = dial(ui, row.cell(), id(&format!("freq{b}")), band.freq as f32, (20.0, 20000.0), true,
            def.freq as f32, hz, "Freq", lit)
        {
            s.eq.bands[b].freq = v.round() as u16;
        }
        if let Some(v) = dial(ui, row.cell(), id(&format!("q{b}")), band.q as f32 / 10.0, (0.7, 5.0), false,
            def.q as f32 / 10.0, |v| format!("{v:.1}"), "Q", lit)
        {
            s.eq.bands[b].q = (v * 10.0).round() as u8;
        }
        row.gap(GAP);
    }

    // low cut, then auto-level
    let y = top_left.y + ROW;
    title(ui, x0, y, "Low cut");
    let mut row = Row { x: x0, y: y + TITLE };
    let lit = s.low_cut.on;
    s.low_cut.on = switch(ui, row.toggle(), key("lowcut"), s.low_cut.on);
    if let Some(v) = dial(ui, row.cell(), id("lowcut-freq"), s.low_cut.freq as f32, (20.0, 500.0), true,
        d.low_cut.freq as f32, hz, "Freq", lit)
    {
        s.low_cut.freq = v.round() as u16;
    }
    let slopes: &'static [&'static str] = &["6 dB/oct", "12 dB/oct", "18 dB/oct", "24 dB/oct"];
    let slope = Choice::new(key("slope"), s.low_cut.slope as usize, slopes);
    look::step_picker(ui, row.step(104.0), &slope);
    if let Some(k) = slope.picked.get() {
        s.low_cut.slope = k as u8;
    }
    row.gap(GROUP_GAP);
    title(ui, row.x, y, "Auto level");
    let lit = s.auto_level.on;
    s.auto_level.on = switch(ui, row.toggle(), key("autolevel"), s.auto_level.on);
    let a = s.auto_level;
    if let Some(v) = dial(ui, row.cell(), id("al-max"), a.max_gain as f32 / 10.0, (0.0, 18.0), false,
        d.auto_level.max_gain as f32 / 10.0, db, "Max Gain", lit)
    {
        s.auto_level.max_gain = (v * 10.0).round() as u16;
    }
    if let Some(v) = dial(ui, row.cell(), id("al-headroom"), a.headroom as f32 / 10.0, (3.0, 12.0), false,
        d.auto_level.headroom as f32 / 10.0, db, "Headroom", lit)
    {
        s.auto_level.headroom = (v * 10.0).round() as u16;
    }
    if let Some(v) = dial(ui, row.cell(), id("al-rise"), a.rise as f32 / 10.0, (0.1, 9.9), false,
        d.auto_level.rise as f32 / 10.0, |v| format!("{v:.1} s"), "Rise Time", lit)
    {
        s.auto_level.rise = (v * 10.0).round() as u16;
    }

    // dynamics
    let y = top_left.y + 2.0 * ROW;
    title(ui, x0, y, "Dynamics");
    let mut row = Row { x: x0, y: y + TITLE };
    let lit = s.dynamics.on;
    s.dynamics.on = switch(ui, row.toggle(), key("dynamics"), s.dynamics.on);
    let (n, dd) = (s.dynamics, d.dynamics);
    let ms = |v: f32| format!("{v:.0} ms");
    let ratio = |v: f32| format!("{v:.1}:1");
    if let Some(v) = dial(ui, row.cell(), id("dyn-gain"), n.gain as f32 / 10.0, (-30.0, 30.0), false,
        dd.gain as f32 / 10.0, db, "Gain", lit)
    {
        s.dynamics.gain = (v * 10.0).round() as i16;
    }
    if let Some(v) = dial(ui, row.cell(), id("dyn-attack"), n.attack as f32, (0.0, 200.0), false, dd.attack as f32, ms,
        "Attack", lit)
    {
        s.dynamics.attack = v.round() as u16;
    }
    if let Some(v) = dial(ui, row.cell(), id("dyn-release"), n.release as f32, (100.0, 999.0), false,
        dd.release as f32, ms, "Release", lit)
    {
        s.dynamics.release = v.round() as u16;
    }
    row.gap(GAP);
    title(ui, row.x, y, "Compressor");
    if let Some(v) = dial(ui, row.cell(), id("comp-threshold"), n.comp_threshold as f32 / 10.0, (-60.0, 0.0), false,
        dd.comp_threshold as f32 / 10.0, db, "Threshold", lit)
    {
        s.dynamics.comp_threshold = (v * 10.0).round() as i16;
    }
    if let Some(v) = dial(ui, row.cell(), id("comp-ratio"), n.comp_ratio as f32 / 10.0, (1.0, 10.0), false,
        dd.comp_ratio as f32 / 10.0, ratio, "Ratio", lit)
    {
        s.dynamics.comp_ratio = (v * 10.0).round() as u8;
    }
    row.gap(GAP);
    title(ui, row.x, y, "Expander");
    if let Some(v) = dial(ui, row.cell(), id("exp-threshold"), n.exp_threshold as f32 / 10.0, (-99.0, -30.0), false,
        dd.exp_threshold as f32 / 10.0, db, "Threshold", lit)
    {
        s.dynamics.exp_threshold = (v * 10.0).round() as i16;
    }
    if let Some(v) = dial(ui, row.cell(), id("exp-ratio"), n.exp_ratio as f32 / 10.0, (1.0, 10.0), false,
        dd.exp_ratio as f32 / 10.0, ratio, "Ratio", lit)
    {
        s.dynamics.exp_ratio = (v * 10.0).round() as u8;
    }
    s.clamped()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_and_choices() {
        assert_eq!((hz(80.0), hz(1000.0), hz(1234.0)), ("80 Hz".into(), "1.00 kHz".into(), "1.23 kHz".into()));
        let c = Choice::new("k".into(), 1, &["Bell", "Shelf", "Low Pass"]);
        assert_eq!(c.position(), 0.5);
        assert_eq!(c.text_at(1.0), "Low Pass");
        assert_eq!(c.step_from(0.5, true), 1.0);
        assert_eq!(c.step_from(0.0, false), 0.0);
        c.set(1.0);
        assert_eq!(c.picked.get(), Some(2));
    }
}
