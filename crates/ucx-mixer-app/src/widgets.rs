//! The mixer's controls that the family's look (`oroboro_egui::widgets`) doesn't have yet: a
//! fader, a level meter and a pan bar. Drawn from the look's colour roles and font only, so every
//! theme works; candidates for the look itself once they've settled (it's a copy here, not edited
//! in this repository).

use std::cell::Cell;

use oroboro_egui::egui::{self, pos2, vec2, Align2, CornerRadius, CursorIcon, Id, Painter, Rect, Sense, Stroke, Ui};
use oroboro_egui::theme::*;
use oroboro_egui::widgets::HostParam;
use ucx_mixer::{LEVEL_MAX, LEVEL_OFF};

/// Fader travel against level: (position, dB) corners, straight in dB between them, and −∞ below
/// the first. As TotalMix's faders: 0 dB about four fifths up, +6 dB at the top.
const TAPER: [(f32, f32); 5] = [(0.02, -65.0), (0.25, -30.0), (0.5, -12.0), (0.82, 0.0), (1.0, 6.0)];

/// The level (tenths of a dB) at a fader position (0 bottom … 1 top).
pub fn level_at(position: f32) -> i16 {
    if position < TAPER[0].0 {
        return LEVEL_OFF;
    }
    for w in TAPER.windows(2) {
        let ((p0, d0), (p1, d1)) = (w[0], w[1]);
        if position <= p1 {
            let db = d0 + (position - p0) / (p1 - p0) * (d1 - d0);
            return ((db * 10.0).round() as i16).clamp(LEVEL_OFF, LEVEL_MAX);
        }
    }
    LEVEL_MAX
}

/// The fader position of a level (tenths of a dB).
pub fn position_of(level: i16) -> f32 {
    if level <= LEVEL_OFF {
        return 0.0;
    }
    let db = level as f32 / 10.0;
    for w in TAPER.windows(2) {
        let ((p0, d0), (p1, d1)) = (w[0], w[1]);
        if db <= d1 {
            return p0 + ((db - d0) / (d1 - d0)).max(0.0) * (p1 - p0);
        }
    }
    1.0
}

/// A level as text: "-10.6", "0.0", "+3.5", "-inf".
pub fn level_text(level: i16) -> String {
    if level <= LEVEL_OFF {
        "-inf".into()
    } else if level > 0 {
        format!("+{:.1}", level as f32 / 10.0)
    } else {
        format!("{:.1}", level as f32 / 10.0)
    }
}

/// A pan or balance as text: "C", "L40", "R100".
pub fn pan_text(pan: i16) -> String {
    match pan {
        0 => "C".into(),
        p if p < 0 => format!("L{}", -p),
        p => format!("R{p}"),
    }
}

/// The fader's cap.
const CAP: egui::Vec2 = vec2(26.0, 12.0);
/// Pixels of drag for the whole travel's worth of position, and with Shift.
const FINE: f32 = 6.0;
/// Wheel steps in tenths of a dB, and with Shift.
const WHEEL_STEP: i16 = 5;
const WHEEL_STEP_FINE: i16 = 1;

/// A vertical fader in `rect` for a level (tenths of a dB). Drag it (Shift for fine), use the wheel
/// (0.5 dB a step, 0.1 with Shift), or set 0 dB with a double-click, right-click or Ctrl-click. `lit`
/// false draws it dimmed (a level that isn't known). Returns the new level when it moved.
pub fn fader(ui: &mut Ui, rect: Rect, id: Id, level: i16, lit: bool) -> Option<i16> {
    let response = ui.interact(rect, id, Sense::click_and_drag());
    let travel = rect.height() - CAP.y;
    let mods = ui.input(|i| i.modifiers);
    let mut new = None;
    if response.double_clicked() || response.secondary_clicked() || (response.clicked() && mods.command) {
        new = Some(0);
    } else if response.dragged() {
        // (the position kept from frame to frame: `drag_delta` is each frame's movement)
        if response.drag_started() {
            ui.data_mut(|d| d.insert_temp(id, position_of(level)));
        }
        let start = ui.data(|d| d.get_temp::<f32>(id)).unwrap_or(position_of(level));
        let scale = if mods.shift { travel * FINE } else { travel };
        let p = (start - response.drag_delta().y / scale).clamp(0.0, 1.0);
        ui.data_mut(|d| d.insert_temp(id, p));
        new = Some(level_at(p));
    }
    if response.hovered() || response.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        let wheel = ui.input(|i| oroboro_egui::widgets::wheel(i).y);
        if wheel != 0.0 && !response.dragged() {
            let step = if mods.shift { WHEEL_STEP_FINE } else { WHEEL_STEP };
            let from = if level <= LEVEL_OFF { -650 } else { level };
            new = Some((from + step * wheel.signum() as i16).clamp(LEVEL_OFF, LEVEL_MAX));
        }
    }
    let was = level;
    let level = new.unwrap_or(level);

    let painter = ui.painter();
    let y_at = |p: f32| rect.bottom() - CAP.y / 2.0 - p * travel;
    let x = rect.center().x;
    let slot = Rect::from_min_max(pos2(x - 2.0, rect.top() + CAP.y / 2.0), pos2(x + 2.0, rect.bottom() - CAP.y / 2.0));
    painter.rect_filled(slot, CornerRadius::same(2), WELL());
    painter.rect_stroke(slot, CornerRadius::same(2), Stroke::new(1.0, LIST_BORDER()), egui::StrokeKind::Outside);
    let zero = y_at(position_of(0));
    for side in [-1.0, 1.0] {
        let a = pos2(x + side * 6.0, zero);
        painter.line_segment([a, a + vec2(side * 5.0, 0.0)], Stroke::new(1.0, FAINT()));
    }
    let cap_y = y_at(position_of(level));
    if lit && level > LEVEL_OFF {
        let fill = Rect::from_min_max(pos2(slot.left(), cap_y), slot.max);
        painter.rect_filled(fill, CornerRadius::same(2), accent_alpha(0x60));
    }
    let cap = Rect::from_center_size(pos2(x, cap_y), CAP);
    let fill = if response.hovered() || response.dragged() { BUTTON_HOVER() } else { BUTTON() };
    let border = if response.dragged() { ACCENT() } else { BUTTON_HOVER_BORDER() };
    painter.rect_filled(cap, CornerRadius::same(3), fill);
    painter.rect_stroke(cap, CornerRadius::same(3), Stroke::new(1.0, border), egui::StrokeKind::Inside);
    let line = if lit { ACCENT() } else { FAINT() };
    painter.line_segment([pos2(cap.left() + 4.0, cap_y), pos2(cap.right() - 4.0, cap_y)], Stroke::new(2.0, line));
    new.filter(|&n| n != was)
}

/// The quietest level a meter shows, dBFS.
pub const METER_FLOOR: f32 = -60.0;

/// A level meter bar in `rect`: RMS filled, the peak a line, from [`METER_FLOOR`] at the bottom to
/// 0 dBFS at the top. The top 6 dB are in the warning colour, a peak at full scale in the error
/// colour.
pub fn meter(painter: &Painter, rect: Rect, rms_db: f32, peak_db: f32) {
    painter.rect_filled(rect, CornerRadius::same(1), WELL());
    let y_at = |db: f32| rect.bottom() - ((db - METER_FLOOR) / -METER_FLOOR).clamp(0.0, 1.0) * rect.height();
    let warn = y_at(-6.0);
    if rms_db > METER_FLOOR {
        let top = y_at(rms_db);
        let low = Rect::from_min_max(pos2(rect.left(), top.max(warn)), rect.max);
        painter.rect_filled(low, CornerRadius::ZERO, ACCENT());
        if top < warn {
            let high = Rect::from_min_max(pos2(rect.left(), top), pos2(rect.right(), warn));
            painter.rect_filled(high, CornerRadius::ZERO, WARNING());
        }
    }
    if peak_db > METER_FLOOR {
        let y = y_at(peak_db);
        let color = if peak_db >= -0.1 {
            ERROR()
        } else if peak_db > -6.0 {
            WARNING()
        } else {
            VALUE()
        };
        painter.line_segment([pos2(rect.left(), y), pos2(rect.right(), y)], Stroke::new(2.0, color));
    }
}

/// A pan (or balance) bar in `rect`, −100 … +100: drag sideways (Shift for fine), double-click or
/// right-click centres it. Returns its response and the new pan when it moved.
pub fn pan_bar(ui: &mut Ui, rect: Rect, id: Id, pan: i16, lit: bool) -> (egui::Response, Option<i16>) {
    let response = ui.interact(rect.expand2(vec2(0.0, 3.0)), id, Sense::click_and_drag());
    let mods = ui.input(|i| i.modifiers);
    let mut new = None;
    if response.double_clicked() || response.secondary_clicked() {
        new = Some(0);
    } else if response.dragged() {
        if response.drag_started() {
            ui.data_mut(|d| d.insert_temp(id, pan as f32));
        }
        let start = ui.data(|d| d.get_temp::<f32>(id)).unwrap_or(pan as f32);
        let scale = if mods.shift { FINE } else { 1.0 };
        let p = (start + response.drag_delta().x / rect.width() * 200.0 / scale).clamp(-100.0, 100.0);
        ui.data_mut(|d| d.insert_temp(id, p));
        new = Some(p.round() as i16);
    }
    if response.hovered() || response.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
    }
    let was = pan;
    let pan = new.unwrap_or(pan);

    let painter = ui.painter();
    let fill = if response.hovered() || response.dragged() { ROW_HOVER() } else { WELL() };
    painter.rect_filled(rect, CornerRadius::same(3), fill);
    painter.rect_stroke(rect, CornerRadius::same(3), Stroke::new(1.0, LIST_BORDER()), egui::StrokeKind::Inside);
    let x_at = |p: i16| rect.center().x + p as f32 / 100.0 * (rect.width() / 2.0 - 2.0);
    let c = rect.center().x;
    painter.line_segment([pos2(c, rect.top() + 2.0), pos2(c, rect.bottom() - 2.0)], Stroke::new(1.0, FAINT()));
    let x = x_at(pan);
    if pan != 0 && lit {
        let span = Rect::from_min_max(pos2(c.min(x), rect.top() + 3.0), pos2(c.max(x), rect.bottom() - 3.0));
        painter.rect_filled(span, CornerRadius::ZERO, accent_alpha(0x70));
    }
    let marker = if lit { ACCENT() } else { FAINT() };
    painter.line_segment([pos2(x, rect.top() + 1.0), pos2(x, rect.bottom() - 1.0)], Stroke::new(2.0, marker));
    (response, new.filter(|&n| n != was))
}

/// The icon that opens a strip's processing, in `rect`: three short sliders, in the accent while
/// any of it is on. Returns its response.
pub fn processing_icon(ui: &mut Ui, rect: Rect, id: Id, on: bool) -> egui::Response {
    let response = ui.interact(rect, id, Sense::click());
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(rect, CornerRadius::same(3), ROW_HOVER());
    }
    let color = if on {
        ACCENT()
    } else if response.hovered() {
        TEXT()
    } else {
        LABEL()
    };
    let c = rect.center();
    for (k, knob) in [(-1.0, 2.0), (0.0, -2.0), (1.0, 1.0)] {
        let y = c.y + k * 4.0;
        painter.line_segment([pos2(c.x - 5.0, y), pos2(c.x + 5.0, y)], Stroke::new(1.0, color));
        painter.circle_filled(pos2(c.x + knob, y), 1.6, color);
    }
    response
}

/// A text centred at `at` in the family's font.
pub fn label(painter: &Painter, at: egui::Pos2, text: &str, size: f32, color: egui::Color32) {
    painter.text(at, Align2::CENTER_CENTER, text, font(size), color);
}

/// An on/off setting the family's pill toggle (`oroboro_egui::widgets::toggle`) can drive: it
/// shows `on`, and a click leaves the new state in `clicked`.
pub struct Flag {
    key: String,
    on: bool,
    pub clicked: Cell<Option<bool>>,
}

impl Flag {
    pub fn new(key: impl Into<String>, on: bool) -> Flag {
        Flag { key: key.into(), on, clicked: Cell::new(None) }
    }
}

impl HostParam for Flag {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn position(&self) -> f32 {
        self.on as u8 as f32
    }
    fn default_position(&self) -> f32 {
        0.0
    }
    fn text_at(&self, position: f32) -> String {
        if position > 0.5 { "On" } else { "Off" }.into()
    }
    fn step_from(&self, position: f32, _up: bool) -> f32 {
        1.0 - position
    }
    fn begin(&self) {}
    fn set(&self, position: f32) {
        self.clicked.set(Some(position > 0.5));
    }
    fn end(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taper_round_trips_and_marks_its_corners() {
        assert_eq!(level_at(0.0), LEVEL_OFF);
        assert_eq!(level_at(1.0), LEVEL_MAX);
        assert_eq!(level_at(0.82), 0);
        assert_eq!(level_at(0.5), -120);
        for level in [-649, -400, -300, -121, -106, -1, 0, 1, 35, 60] {
            assert_eq!(level_at(position_of(level)), level, "{level}");
        }
        let mut last = position_of(LEVEL_OFF);
        for level in LEVEL_OFF + 1..=LEVEL_MAX {
            let p = position_of(level);
            assert!(p > last, "rising at {level}");
            last = p;
        }
    }

    #[test]
    fn texts() {
        assert_eq!(level_text(-106), "-10.6");
        assert_eq!(level_text(0), "0.0");
        assert_eq!(level_text(35), "+3.5");
        assert_eq!(level_text(LEVEL_OFF), "-inf");
        assert_eq!((pan_text(0), pan_text(-40), pan_text(100)), ("C".into(), "L40".into(), "R100".into()));
    }
}
