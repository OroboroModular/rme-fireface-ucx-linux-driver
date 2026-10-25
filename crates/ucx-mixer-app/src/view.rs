//! The mixer's window: TotalMix's three rows of strips (hardware inputs, playback, hardware
//! outputs) in the family's sections. Each output strip has a submix; the inputs and playback
//! rows show their sends into the selected one, and a click on an output's name selects it.
//! Layouts are fixed-pixel, as in the family.
//!
//! Every change goes into the model and makes register words ([`MixerUi::apply`]). What they may
//! reach is chosen in the top bar ([`Reach`]): nothing at first; "Send to unit" sends the words
//! for the digital outputs (SPDIF, ADAT), where nothing is plugged in; "Analog outputs", after a
//! card that says to turn the monitors and phones down, sends the rest too. Words the reach
//! doesn't allow are held back: kept, the latest value for each register, until "Send held
//! changes" sends the ones the reach then allows. Nothing held goes out by itself.

use std::collections::BTreeMap;

use oroboro_egui::egui::{pos2, vec2, Align2, CornerRadius, Id, Rect, Sense, Ui, Vec2};
use oroboro_egui::theme::*;
use oroboro_egui::widgets::{self as look, ButtonKind};
use ucx_mixer::{Mixer, Send, Settings, Source, Strip, INPUTS, LEVEL_OFF, OUTPUTS, PLAYBACK};
use ucx_protocol::regs::CHANNELS;
use ucx_protocol::word::Word;

use crate::processing::{self, Edit};
use crate::snapshots::{self, Snapshot, SLOTS};
use crate::unit::{Levels, Reach, Status, ROWS};
use crate::widgets::{self, Flag};

/// The window, in points.
pub const SIZE: Vec2 = vec2(948.0, 826.0);
const MARGIN: f32 = 12.0;
const TOP: f32 = 64.0;
const GAP: f32 = 8.0;
const SECTION: f32 = 236.0;
/// A strip's width, and the step from one to the next.
const STRIP: f32 = 76.0;
const PITCH: f32 = 80.0;
/// Inside a strip, from the top of the section's body: the name, the pan bar's centre, the
/// fader, the value line's centre, the mute toggle.
const NAME: f32 = 18.0;
const PAN_Y: f32 = 26.0;
const FADER_Y: f32 = 38.0;
const FADER_H: f32 = 112.0;
const VALUE_Y: f32 = 162.0;
const MUTE_Y: f32 = 174.0;
/// The processing icon at the right of an input's or output's name.
const ICON: f32 = 18.0;
/// The top bar's controls, right to left: Send to unit, Analog outputs, Send whole mix,
/// Snapshots, the theme.
const CONTROLS: [f32; 5] = [120.0, 132.0, 118.0, 104.0, 96.0];
/// The card before the analog outputs are reached.
const ASK: Vec2 = vec2(440.0, 196.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Row {
    Inputs,
    Playback,
    Outputs,
}

impl Row {
    fn strips(self) -> &'static [Strip] {
        match self {
            Row::Inputs => &INPUTS,
            Row::Playback => &PLAYBACK,
            Row::Outputs => &OUTPUTS,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Row::Inputs => "Hardware inputs",
            Row::Playback => "Playback",
            Row::Outputs => "Hardware outputs",
        }
    }

    /// Its meters' row in [`Levels`].
    fn meters(self) -> usize {
        self as usize
    }

    fn source(self, i: usize) -> Option<Source> {
        match self {
            Row::Inputs => Some(Source::Input(i)),
            Row::Playback => Some(Source::Playback(i)),
            Row::Outputs => None,
        }
    }
}

/// Meters as shown: the unit's levels, falling back slowly when they drop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meters {
    pub peak: [[f32; CHANNELS]; ROWS],
    pub rms: [[f32; CHANNELS]; ROWS],
}

impl Meters {
    pub const SILENT: Meters =
        Meters { peak: [[f32::NEG_INFINITY; CHANNELS]; ROWS], rms: [[f32::NEG_INFINITY; CHANNELS]; ROWS] };

    /// Takes the unit's new levels, if any, and lets the shown ones fall `dt` seconds' worth:
    /// peaks 20 dB a second, RMS 40.
    pub fn update(&mut self, new: Option<&Levels>, dt: f32) {
        let fall = |shown: f32, rate: f32| {
            let v = shown - rate * dt;
            if v < widgets::METER_FLOOR {
                f32::NEG_INFINITY
            } else {
                v
            }
        };
        for r in 0..ROWS {
            for c in 0..CHANNELS {
                self.peak[r][c] = fall(self.peak[r][c], 20.0);
                self.rms[r][c] = fall(self.rms[r][c], 40.0);
                if let Some(n) = new {
                    self.peak[r][c] = self.peak[r][c].max(n.peak[r][c]);
                    self.rms[r][c] = self.rms[r][c].max(n.rms[r][c]);
                }
            }
        }
    }
}

/// Everything the window shows.
pub struct MixerUi {
    pub mixer: Mixer,
    /// The output strip whose submix the inputs and playback show.
    pub submix: usize,
    pub meters: Meters,
    pub status: Status,
    /// What changes may reach; nothing at first.
    pub reach: Reach,
    /// Words to send, taken by the caller after each frame.
    pub outgoing: Vec<Word>,
    /// Changes the reach held back (or made while the unit was away): the latest value of each
    /// register ([`slot`]).
    pub held: BTreeMap<(u16, u16), u16>,
    /// Words the link lost (queued when the unit went away).
    pub lost: u64,
    /// The card asking before the analog outputs are reached is open.
    pub asking: bool,
    /// The strip whose processing card is open.
    pub editing: Option<Edit>,
    /// The snapshots' slots, and whether their card is open.
    pub snapshots: Vec<Option<Snapshot>>,
    pub snapping: bool,
    /// Where the mix came from, shown under the window.
    pub origin: String,
}

impl MixerUi {
    /// Starting from the mix TotalMix left in the unit, on the first submix, sending nothing.
    pub fn new() -> MixerUi {
        MixerUi::with(Mixer::totalmix(), 0, "TotalMix's state after power-on (captured 2026-10-23)".into())
    }

    pub fn with(mixer: Mixer, submix: usize, origin: String) -> MixerUi {
        MixerUi {
            mixer,
            submix: submix.min(OUTPUTS.len() - 1),
            meters: Meters::SILENT,
            status: Status::Searching,
            reach: Reach::Nothing,
            outgoing: Vec::new(),
            held: BTreeMap::new(),
            lost: 0,
            asking: false,
            editing: None,
            snapshots: vec![None; SLOTS],
            snapping: false,
            origin,
        }
    }

    /// A change's words: queued when the reach allows them (a held value for the same register is
    /// then out of date), else held back.
    pub fn apply(&mut self, words: Vec<Word>) {
        for w in words {
            if self.reach.allows(w) {
                self.held.remove(&slot(w));
                self.outgoing.push(w);
            } else {
                self.held.insert(slot(w), w.value);
            }
        }
    }

    /// Words that couldn't go out (no unit): held back, to be sent on purpose later.
    pub fn hold(&mut self, words: Vec<Word>) {
        for w in words {
            self.held.insert(slot(w), w.value);
        }
    }

    /// The held changes the reach allows now.
    pub fn held_ready(&self) -> usize {
        self.held.iter().filter(|(&(addr, _), &v)| self.reach.allows(Word::new(addr, v))).count()
    }

    /// Sends the held changes the reach allows now; the rest stay held.
    pub fn send_held(&mut self) {
        let reach = self.reach;
        let ready: Vec<Word> =
            self.held.iter().map(|(&(addr, _), &v)| Word::new(addr, v)).filter(|w| reach.allows(*w)).collect();
        for w in &ready {
            self.held.remove(&slot(*w));
        }
        self.outgoing.extend(ready);
    }

    pub fn change_send(&mut self, submix: usize, source: Source, send: Send) {
        let words = self.mixer.set_send(submix, source, send);
        self.apply(words);
    }

    pub fn change_mute(&mut self, source: Source, mute: bool) {
        let words = self.mixer.set_mute(source, mute);
        self.apply(words);
    }

    pub fn change_output_level(&mut self, output: usize, level: i16) {
        let words = self.mixer.set_output_level(output, level);
        self.apply(words);
    }

    pub fn change_output_mute(&mut self, output: usize, mute: bool) {
        let words = self.mixer.set_output_mute(output, mute);
        self.apply(words);
    }

    pub fn change_input_settings(&mut self, input: usize, settings: Settings) {
        let words = self.mixer.set_input_settings(input, settings);
        self.apply(words);
    }

    pub fn change_image(&mut self, source: Source, image: ucx_mixer::Image) {
        let words = self.mixer.set_image(source, image);
        self.apply(words);
    }

    pub fn change_preamp(&mut self, input: usize, preamp: ucx_mixer::Preamp) {
        let words = self.mixer.set_preamp(input, preamp);
        self.apply(words);
    }

    pub fn change_output_settings(&mut self, output: usize, settings: Settings) {
        let words = self.mixer.set_output_settings(output, settings);
        self.apply(words);
    }

    /// The whole mix again, as far as the reach goes: after it widened, or when the unit was
    /// changed from elsewhere.
    pub fn send_whole_mix(&mut self) {
        let words = self.mixer.words();
        self.apply(words);
    }
}

/// Where a word's value lives: its register, and for a fader table entry which of its two
/// fields (bit 3 of the value).
fn slot(w: Word) -> (u16, u16) {
    let (channel, off) = (w.addr >> 8, w.addr & 0xff);
    let table = (0x12..0x24).contains(&channel) && off >= 0xe0;
    (w.addr, if table { w.value & 0x8 } else { 0 })
}

impl Default for MixerUi {
    fn default() -> Self {
        Self::new()
    }
}

/// Draws the window into `ui`'s whole rect and takes its input.
pub fn draw(ui: &mut Ui, s: &mut MixerUi) {
    let area = ui.max_rect();
    ui.painter().rect_filled(area, CornerRadius::ZERO, WINDOW());
    top_bar(ui, s, Rect::from_min_size(area.min + vec2(MARGIN, 0.0), vec2(SIZE.x - 2.0 * MARGIN, TOP)));
    let mut y = area.top() + TOP;
    for row in [Row::Inputs, Row::Playback, Row::Outputs] {
        let rect = Rect::from_min_size(pos2(area.left() + MARGIN, y), vec2(SIZE.x - 2.0 * MARGIN, SECTION));
        section(ui, s, row, rect);
        y += SECTION + GAP;
    }
    let painter = ui.painter();
    let note = format!("Mix: {}. The unit can't be read back.", s.origin);
    painter.text(pos2(area.left() + MARGIN + 2.0, y + 2.0), Align2::LEFT_TOP, note, font(10.0), MUTED());
    let (reach, color) = match s.reach {
        Reach::Nothing => ("Not sending: changes stay in this window".to_string(), WARNING()),
        Reach::Digital => ("Sending to the SPDIF and ADAT outputs only".to_string(), ACCENT_SOFT()),
        Reach::All => ("Sending to every output".to_string(), ACCENT()),
    };
    let mut reach = match (s.held.len(), s.lost) {
        (0, 0) => reach,
        (0, lost) => format!("{reach} · {lost} words lost when the unit went away"),
        (held, 0) => format!("{reach} · {held} changes held back"),
        (held, lost) => format!("{reach} · {held} held back, {lost} lost"),
    };
    let mut right = area.left() + SIZE.x - MARGIN - 2.0;
    let ready = s.held_ready();
    if ready > 0 {
        let button = Rect::from_min_size(pos2(right - 150.0, y + 1.0), vec2(150.0, 22.0));
        if look::button(ui, button, &format!("Send {ready} held changes"), ButtonKind::Primary, false, "ucx-held").clicked()
        {
            s.send_held();
        }
        right -= 160.0;
        reach += " ·";
    }
    ui.painter().text(pos2(right, y + 6.0), Align2::RIGHT_TOP, reach, font(10.0), color);
    if let Some(edit) = s.editing {
        processing::card(ui, s, edit, area);
    }
    if s.snapping {
        snapshots::card(ui, s, area);
    }
    if s.asking {
        ask_analog(ui, s, area);
    }
}

fn top_bar(ui: &mut Ui, s: &mut MixerUi, rect: Rect) {
    look::brand(ui, Rect::from_min_size(rect.min + vec2(0.0, 6.0), vec2(220.0, 40.0)), "UCX Mixer");
    let (text, color) = match &s.status {
        Status::Searching => ("Looking for the Fireface UCX…".to_string(), MUTED()),
        Status::Absent(why) => (format!("No Fireface UCX: {why}"), FAINT()),
        Status::Busy(why) => (format!("Mixer interface busy: {why}"), WARNING()),
        Status::Live { rate } => {
            let rate = rate.map_or("rate unknown".into(), |r| format!("{:.1} kHz", r as f32 / 1000.0));
            (format!("Fireface UCX · {rate} · meters live"), TEXT())
        }
    };
    let right = rect.right() - 2.0;
    let text = look::fit(ui, &text, &font(11.0), rect.width() - 260.0);
    ui.painter().text(pos2(right, rect.top() + 14.0), Align2::RIGHT_CENTER, text, font(11.0), color);

    // the controls, right to left
    let mut x = right;
    let [send_w, analog_w, whole_w, snapshots_w, theme_w] = CONTROLS;
    let mut next = |w: f32| {
        x -= w;
        let r = Rect::from_min_size(pos2(x, rect.top() + 30.0), vec2(w, 24.0));
        x -= 8.0;
        r
    };
    let sending = Flag::new("ucx-send", s.reach != Reach::Nothing);
    look::toggle(ui, next(send_w), &sending, "Send to unit");
    if let Some(on) = sending.clicked.get() {
        s.reach = if on { Reach::Digital } else { Reach::Nothing };
    }
    let analog = Flag::new("ucx-analog", s.reach == Reach::All);
    look::toggle(ui, next(analog_w), &analog, "Analog outputs");
    match analog.clicked.get() {
        Some(true) => s.asking = true,
        Some(false) => s.reach = Reach::Digital,
        None => {}
    }
    if look::button(ui, next(whole_w), "Send whole mix", ButtonKind::Panel, false, "ucx-whole").clicked() {
        s.send_whole_mix();
    }
    if look::list_button(ui, next(snapshots_w), "Snapshots", "ucx-snapshots").clicked() {
        s.snapping = true;
    }
    let theme = current().name;
    if look::list_button(ui, next(theme_w), &theme, "ucx-theme").clicked() {
        let themes = Theme::built_in();
        let i = themes.iter().position(|t| t.name == theme).map_or(0, |i| (i + 1) % themes.len());
        apply(&themes[i]);
    }
}

/// The card before the analog outputs are reached.
fn ask_analog(ui: &mut Ui, s: &mut MixerUi, area: Rect) {
    look::overlay(ui.ctx(), "ucx-analog", area, |ui| {
        let rect = look::card_rect(ui, area, ASK, "ucx-analog");
        let body = look::card(ui, rect, "Send to the analog outputs");
        let painter = ui.painter();
        let text = "Changes will then reach the analog outputs AN 1–6 and the phones (PH 7/8): whatever is \
                    plugged in there can get loud. Turn monitors and headphones down before moving anything. \
                    Changes made meanwhile were held back; the button beside their count sends them.";
        let galley = painter.layout(text.into(), font(12.0), TEXT(), body.width());
        painter.galley(body.min, galley, TEXT());
        let buttons = body.bottom() - 28.0;
        let cancel = Rect::from_min_size(pos2(body.right() - 290.0, buttons), vec2(100.0, 28.0));
        let go = Rect::from_min_size(pos2(body.right() - 180.0, buttons), vec2(180.0, 28.0));
        if look::button(ui, cancel, "Cancel", ButtonKind::Panel, false, "ucx-analog-cancel").clicked() {
            s.asking = false;
        }
        if look::button(ui, go, "Send to analog too", ButtonKind::Primary, false, "ucx-analog-go").clicked() {
            s.asking = false;
            s.reach = Reach::All;
        }
    });
}

fn section(ui: &mut Ui, s: &mut MixerUi, row: Row, rect: Rect) {
    let body = look::section(ui, rect, row.title());
    let note = match row {
        Row::Outputs => "Click a name for its submix".to_string(),
        _ => format!("Into {}", OUTPUTS[s.submix].name),
    };
    let at = pos2(rect.right() - 12.0, rect.top() + 17.0);
    ui.painter().text(at, Align2::RIGHT_CENTER, note.to_uppercase(), font(9.0), MUTED());
    for (i, strip) in row.strips().iter().enumerate() {
        let r = Rect::from_min_size(pos2(body.left() + i as f32 * PITCH, body.top()), vec2(STRIP, body.height()));
        strip_ui(ui, s, row, i, *strip, r);
    }
}

fn strip_ui(ui: &mut Ui, s: &mut MixerUi, row: Row, i: usize, strip: Strip, r: Rect) {
    let id = Id::new(("ucx-strip", row as u8, i));
    let edit = match row {
        Row::Inputs => Some(Edit::Input(i)),
        Row::Playback => Some(Edit::Playback(i)),
        Row::Outputs => Some(Edit::Output(i)),
    };
    // (an icon at the name's right opens the strip's processing card)
    let name_w = if edit.is_some() { STRIP - ICON } else { STRIP };
    let name_rect = Rect::from_min_size(r.min, vec2(name_w, NAME));
    let mut selected = false;
    if row == Row::Outputs {
        let response = ui.interact(name_rect, id.with("name"), Sense::click());
        if response.clicked() {
            s.submix = i;
        }
        selected = s.submix == i;
        let painter = ui.painter();
        if selected {
            painter.rect_filled(r.expand2(vec2(2.0, 0.0)), CornerRadius::same(6), SELECTED());
        } else if response.hovered() {
            painter.rect_filled(name_rect, CornerRadius::same(4), ROW_HOVER());
        }
    }
    let name_color = if selected { ACCENT_SOFT() } else { TEXT() };
    let name = look::fit(ui, strip.name, &font(11.0), name_w - 4.0);
    let at = if edit.is_some() { pos2(r.center().x - ICON / 2.0 + 4.0, name_rect.center().y) } else { name_rect.center() };
    widgets::label(ui.painter(), at, &name, 11.0, name_color);
    if let Some(edit) = edit {
        let settings = match edit {
            Edit::Input(i) => Some(s.mixer.input_settings[i]),
            Edit::Output(o) => Some(s.mixer.output_settings[o]),
            Edit::Playback(_) => None,
        };
        let image = match edit {
            Edit::Input(i) => s.mixer.input_image[i],
            Edit::Playback(p) => s.mixer.playback_image[p],
            Edit::Output(_) => ucx_mixer::Image::PLAIN,
        };
        let on = settings.is_some_and(|t| t.eq.on || t.low_cut.on || t.dynamics.on || t.auto_level.on)
            || image != ucx_mixer::Image::PLAIN;
        let icon = Rect::from_min_size(pos2(r.right() - ICON, r.top()), vec2(ICON, NAME));
        if widgets::processing_icon(ui, icon, id.with("processing"), on).clicked() {
            s.editing = Some(edit);
        }
    }

    let fader_rect = Rect::from_min_size(pos2(r.left() + 8.0, r.top() + FADER_Y), vec2(34.0, FADER_H));
    let mute_rect = Rect::from_center_size(pos2(r.center().x, r.top() + MUTE_Y + 10.0), vec2(64.0, 20.0));
    let mut value;
    let muted;
    if let Some(source) = row.source(i) {
        let send = s.mixer.send(s.submix, source);
        muted = s.mixer.muted(source);
        value = widgets::level_text(send.level);
        let pan_rect = Rect::from_center_size(pos2(r.center().x, r.top() + PAN_Y), vec2(56.0, 10.0));
        let (pan, new_pan) = widgets::pan_bar(ui, pan_rect, id.with("pan"), send.pan, !muted);
        if let Some(p) = new_pan {
            s.change_send(s.submix, source, Send { pan: p, ..send });
        }
        if pan.hovered() || pan.dragged() {
            value = widgets::pan_text(s.mixer.send(s.submix, source).pan);
        }
        if let Some(level) = widgets::fader(ui, fader_rect, id.with("fader"), send.level, !muted) {
            s.change_send(s.submix, source, Send { level, ..send });
            value = widgets::level_text(level);
        }
        let flag = Flag::new(format!("ucx-mute-{}-{i}", row as u8), muted);
        look::toggle(ui, mute_rect, &flag, "Mute");
        if let Some(m) = flag.clicked.get() {
            s.change_mute(source, m);
        }
    } else {
        let out = s.mixer.outputs[i];
        muted = out.mute;
        // (the unit keeps its output levels; until set here they're unknown)
        value = out.level.map_or("?".into(), widgets::level_text);
        // (unknown: the fader starts at the bottom, so the first move can only bring it up from
        // silence, never jump it to 0 dB)
        let shown = out.level.unwrap_or(LEVEL_OFF);
        if let Some(level) = widgets::fader(ui, fader_rect, id.with("fader"), shown, out.level.is_some())
        {
            s.change_output_level(i, level);
            value = widgets::level_text(level);
        }
        let flag = Flag::new(format!("ucx-mute-out-{i}"), muted);
        look::toggle(ui, mute_rect, &flag, "Mute");
        if let Some(m) = flag.clicked.get() {
            s.change_output_mute(i, m);
        }
    }

    // meters beside the fader, its travel's height
    let painter = ui.painter();
    let top = fader_rect.top() + 6.0;
    let bottom = fader_rect.bottom() - 6.0;
    let channels: Vec<usize> = strip.channels().map(usize::from).collect();
    let width = if channels.len() == 1 { 8.0 } else { 5.0 };
    for (k, &c) in channels.iter().enumerate() {
        let x = r.left() + 50.0 + k as f32 * (width + 3.0);
        let bar = Rect::from_min_max(pos2(x, top), pos2(x + width, bottom));
        let m = row.meters();
        widgets::meter(painter, bar, s.meters.rms[m][c], s.meters.peak[m][c]);
    }
    let color = if muted { FAINT() } else { VALUE() };
    widgets::label(painter, pos2(r.center().x, r.top() + VALUE_Y), &value, 10.0, color);
}

#[cfg(test)]
mod tests {
    use super::*;
    use oroboro_egui::egui::{Event, Modifiers, PointerButton, Pos2};
    use oroboro_egui::snapshot::Snapshot;

    fn snapshot(theme: &Theme) -> Snapshot {
        oroboro_egui::touch::set_here(Some(false));
        apply_here(theme);
        let snap = Snapshot::new(SIZE, 1.0);
        install(&snap.ctx);
        snap
    }

    /// A window with a little of everything on it: meters, a panned send, an output level.
    fn busy() -> MixerUi {
        let mut s = MixerUi::new();
        s.status = Status::Live { rate: Some(48000) };
        for (row, c, rms, peak) in [(0, 0, -30.0, -22.0), (0, 6, -14.0, -8.0), (0, 7, -15.0, -9.5), (1, 0, -9.0, -3.0),
            (1, 1, -10.0, -4.0), (2, 0, -12.0, -5.0), (2, 1, -13.0, -0.05), (2, 6, -20.0, -14.0), (2, 7, -21.0, -15.0)]
        {
            s.meters.rms[row][c] = rms;
            s.meters.peak[row][c] = peak;
        }
        s.mixer.set_send(0, Source::Input(0), Send { level: -60, pan: -40 });
        s.mixer.set_output_level(0, -100);
        s.mixer.set_mute(Source::Input(3), true);
        s
    }

    fn picture(theme: Theme, name: &str, s: &mut MixerUi) {
        let mut snap = snapshot(&theme);
        let image = snap.settle(|ui| draw(ui, s));
        assert_eq!((image.width, image.height), (SIZE.x as usize, SIZE.y as usize));
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/pictures");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(format!("{name}.png")), image.to_png()).unwrap();
    }

    #[test]
    fn pictures() {
        picture(Theme::dusk(), "ucx-mixer-dusk", &mut busy());
        picture(Theme::paper(), "ucx-mixer-paper", &mut busy());
        let mut asking = busy();
        asking.reach = Reach::Digital;
        let words = asking.mixer.set_send(0, Source::Playback(1), Send { level: 0, pan: 0 });
        asking.hold(words);
        asking.asking = true;
        picture(Theme::dusk(), "ucx-mixer-asking", &mut asking);
        let mut editing = busy();
        let mut spdif = Settings::TOTALMIX;
        spdif.eq.on = true;
        spdif.eq.bands[1].gain = 45;
        spdif.dynamics.on = true;
        spdif.dynamics.comp_ratio = 30;
        editing.mixer.set_output_settings(4, spdif);
        editing.editing = Some(Edit::Output(4));
        picture(Theme::dusk(), "ucx-mixer-processing", &mut editing);
        let mut preamp = busy();
        preamp.mixer.set_preamp(2, ucx_mixer::Preamp { gain: Some(80), hi_z: true, autoset: false });
        preamp.editing = Some(Edit::Input(2));
        picture(Theme::dusk(), "ucx-mixer-preamp", &mut preamp);
        let mut stereo = busy();
        stereo.mixer.set_image(Source::Playback(0), ucx_mixer::Image { phase: [false, true], width: 0 });
        stereo.editing = Some(Edit::Playback(0));
        picture(Theme::dusk(), "ucx-mixer-playback", &mut stereo);
        let mut snapping = busy();
        snapping.store(0, "Tracking".into());
        snapping.store(3, "Mixdown, phones".into());
        snapping.snapping = true;
        picture(Theme::dusk(), "ucx-mixer-snapshots", &mut snapping);
    }

    fn press(at: Pos2) -> Vec<Event> {
        vec![
            Event::PointerMoved(at),
            Event::PointerButton { pos: at, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE },
        ]
    }

    fn release(at: Pos2) -> Vec<Event> {
        vec![Event::PointerButton { pos: at, button: PointerButton::Primary, pressed: false, modifiers: Modifiers::NONE }]
    }

    fn click(snap: &mut Snapshot, s: &mut MixerUi, at: Pos2) {
        snap.step(press(at), |ui| draw(ui, s));
        snap.step(release(at), |ui| draw(ui, s));
    }

    /// Where a row's strip `i` starts: its body's top left.
    fn strip_at(row: usize, i: usize) -> Pos2 {
        let top = TOP + row as f32 * (SECTION + GAP) + look::section_head();
        pos2(MARGIN + 12.0 + i as f32 * PITCH, top)
    }

    /// The middle of the top bar's control `k`, right to left (`CONTROLS`).
    fn control(k: usize) -> Pos2 {
        let right = SIZE.x - MARGIN - 2.0;
        let x = right - CONTROLS[..k].iter().map(|w| w + 8.0).sum::<f32>() - CONTROLS[k] / 2.0;
        pos2(x, 30.0 + 12.0)
    }

    #[test]
    fn dragging_a_fader_sets_the_send_in_the_selected_submix() {
        let mut snap = snapshot(&Theme::dusk());
        let mut s = MixerUi::new();
        s.submix = 2;
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        // Mic 1's cap sits at the bottom (−∞): drag it up 60 points
        let cap = strip_at(0, 0) + vec2(8.0 + 17.0, FADER_Y + FADER_H - 6.0);
        snap.step(press(cap), |ui| draw(ui, &mut s));
        for k in 1..=6 {
            snap.step(vec![Event::PointerMoved(cap - vec2(0.0, 10.0 * k as f32))], |ui| draw(ui, &mut s));
        }
        snap.step(release(cap - vec2(0.0, 60.0)), |ui| draw(ui, &mut s));
        let send = s.mixer.send(2, Source::Input(0));
        assert_eq!(send.level, widgets::level_at(60.0 / (FADER_H - 12.0)));
        assert_eq!(s.mixer.send(0, Source::Input(0)), Send::OFF, "other submixes untouched");
        assert!(s.outgoing.is_empty() && !s.held.is_empty(), "not sending: held back");
    }

    #[test]
    fn clicking_an_output_name_selects_its_submix_and_mute_mutes() {
        let mut snap = snapshot(&Theme::dusk());
        let mut s = MixerUi::new();
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, strip_at(2, 3) + vec2(STRIP / 2.0, NAME / 2.0));
        assert_eq!(s.submix, 3, "PH 7/8");
        click(&mut snap, &mut s, strip_at(1, 0) + vec2(STRIP / 2.0, MUTE_Y + 10.0));
        assert!(s.mixer.muted(Source::Playback(0)));
    }

    #[test]
    fn the_processing_card_opens_from_the_icon_and_sends_as_far_as_the_reach_goes() {
        let mut snap = snapshot(&Theme::dusk());
        let mut s = MixerUi::new();
        s.reach = Reach::Digital;
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        // SPDIF in the outputs row: its icon
        click(&mut snap, &mut s, strip_at(2, 4) + vec2(STRIP - ICON / 2.0, NAME / 2.0));
        assert_eq!(s.editing, Some(Edit::Output(4)));
        assert_eq!(s.submix, 0, "the icon isn't the name: the submix stays");
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        // the EQ's On toggle: the card's body's top left, under the group title
        let card = Rect::from_center_size(pos2(SIZE.x / 2.0, SIZE.y / 2.0), processing::CARD);
        let body = pos2(card.left() + 24.0, card.top() + 20.0 + 14.0 + 12.0);
        click(&mut snap, &mut s, body + vec2(32.0, 18.0 + 14.0 + 11.0));
        assert!(s.mixer.output_settings[4].eq.on, "EQ on");
        assert_eq!(s.outgoing, vec![Word::new(0x1a40, 1), Word::new(0x1b40, 1)], "a digital output's: sent");
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, pos2(card.right() - 24.0 - 50.0, card.bottom() - 20.0 - 14.0));
        assert_eq!(s.editing, None, "Done");

        // an input's processing is held back unless the analog outputs may be reached
        s.outgoing.clear();
        let mut on = Settings::TOTALMIX;
        on.low_cut.on = true;
        s.change_input_settings(0, on);
        assert!(s.outgoing.is_empty() && s.held.len() == 1);
    }

    #[test]
    fn an_unknown_output_level_comes_up_from_silence() {
        let mut snap = snapshot(&Theme::dusk());
        let mut s = MixerUi::new();
        s.reach = Reach::All;
        assert_eq!(s.mixer.outputs[3].level, None, "PH 7/8 unknown");
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        // the cap of an unknown level sits at the bottom: nudge it up 12 points
        let cap = strip_at(2, 3) + vec2(8.0 + 17.0, FADER_Y + FADER_H - 6.0);
        snap.step(press(cap), |ui| draw(ui, &mut s));
        for k in 1..=3 {
            snap.step(vec![Event::PointerMoved(cap - vec2(0.0, 4.0 * k as f32))], |ui| draw(ui, &mut s));
        }
        snap.step(release(cap - vec2(0.0, 12.0)), |ui| draw(ui, &mut s));
        let level = s.mixer.outputs[3].level.expect("set by the drag");
        assert!(level < -300, "a small move from the bottom stays quiet: {level}");
    }

    #[test]
    fn held_changes_keep_the_latest_value_and_go_out_when_asked() {
        let mut s = MixerUi::new();
        s.change_send(4, Source::Playback(1), Send { level: -200, pan: 0 });
        s.change_send(4, Source::Playback(1), Send { level: -60, pan: 0 });
        s.change_send(0, Source::Playback(1), Send { level: -60, pan: 0 });
        assert!(s.outgoing.is_empty());
        assert_eq!(s.held.len(), 8, "two strips' four matrix entries, the latest values");
        assert_eq!(s.held_ready(), 0, "not sending");
        s.reach = Reach::Digital;
        assert_eq!(s.held_ready(), 4, "the digital submix's");
        s.send_held();
        assert_eq!(s.outgoing.len(), 4);
        assert!(s.outgoing.iter().all(|w| ucx_mixer::digital_only(*w)));
        let gain = s.outgoing.iter().find(|w| w.addr == ucx_protocol::regs::matrix_reg(8, ucx_protocol::regs::Source::Playback(2)));
        assert!((ucx_protocol::values::MatrixGain(gain.unwrap().value).db() + 6.0).abs() < 0.06, "the latest value");
        assert_eq!(s.held.len(), 4, "the analog submix's stay held");
    }

    #[test]
    fn sending_reaches_the_digital_outputs_and_asks_before_the_analog_ones() {
        let mut snap = snapshot(&Theme::dusk());
        let mut s = MixerUi::new();
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, control(0));
        assert_eq!(s.reach, Reach::Digital, "Send to unit");
        s.change_send(4, Source::Playback(1), Send { level: -60, pan: 0 });
        assert!(!s.outgoing.is_empty() && s.outgoing.iter().all(|w| ucx_mixer::digital_only(*w)));
        let queued = s.outgoing.len();
        s.change_send(0, Source::Playback(1), Send { level: 0, pan: 0 });
        assert_eq!((s.outgoing.len(), s.held.len()), (queued, 4), "an analog submix's words are held");

        click(&mut snap, &mut s, control(1));
        assert!(s.asking && s.reach == Reach::Digital, "the card asks first");
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        // the card's buttons: Cancel, then ask again and say yes
        let card = Rect::from_center_size(pos2(SIZE.x / 2.0, SIZE.y / 2.0), ASK);
        let y = card.bottom() - 20.0 - 14.0;
        click(&mut snap, &mut s, pos2(card.right() - 24.0 - 240.0, y));
        assert!(!s.asking && s.reach == Reach::Digital, "cancelled");
        // (egui hit-tests against the frame before, which still had the card over everything)
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, control(1));
        assert!(s.asking, "asks again");
        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, pos2(card.right() - 24.0 - 90.0, y));
        assert_eq!(s.reach, Reach::All, "sent to analog too");
        s.outgoing.clear();
        s.send_whole_mix();
        assert_eq!(s.outgoing.len(), s.mixer.words().len());

        snap.settle_undrawn(|ui| draw(ui, &mut s));
        click(&mut snap, &mut s, control(0));
        assert_eq!(s.reach, Reach::Nothing, "off again");
    }
}
