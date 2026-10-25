//! The family's controls and frames (PLUGIN_GUIDE.md "Geometry",
//! "Controls", "Overlays"), drawn into rects the plugin lays out.
//!
//! Every param change is wrapped in begin/end set parameter, so hosts record
//! one undo step per gesture. A host's parameter reaches the widgets
//! through `HostParam`, so they don't depend on a plugin framework.

use std::f32::consts::PI;
use std::hash::Hash;

use egui::{
    self, epaint, pos2, vec2, Align2, Color32, CornerRadius, CursorIcon, FontId, Id, Painter, Pos2, Rect, Response,
    Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
};

use crate::theme::*;
use crate::touch;

/// A knob's diameter, and the cell it sits in: knob, value (16 px, 4 px
/// below the knob), label (16 px).
pub const KNOB: f32 = 50.0;
pub const KNOB_CELL: Vec2 = vec2(64.0, KNOB + 4.0 + 16.0 + 16.0);

/// A knob's cell with a mouse (`KNOB_CELL`) or with fingers, whose value
/// and label are larger.
pub fn knob_cell() -> Vec2 {
    touch::pick(KNOB_CELL, vec2(76.0, KNOB + 4.0 + 18.0 + 18.0))
}

/// The lines under a knob, its value's and its label's: taller with
/// fingers, whose text is larger.
fn knob_line() -> f32 {
    touch::pick(16.0, 18.0)
}

/// A cell `width` wide for a knob `diameter` across (an iPad's macros are
/// larger than `KNOB`): the knob is as big as its cell's height leaves
/// over its value and its label.
pub fn knob_cell_for(diameter: f32, width: f32) -> Vec2 {
    vec2(width, diameter + 4.0 + 2.0 * knob_line())
}
/// Pill toggles; stack them in a 104 px column.
pub const TOGGLE: Vec2 = vec2(104.0, 26.0);
/// The recessed step picker.
pub const STEP: Vec2 = vec2(72.0, 30.0);

const RING_WIDTH: f32 = 4.0;
/// The knob's 270° arc opens at the bottom: 7 o'clock to 5 o'clock.
const ARC_START: f32 = PI * 0.75;
const ARC_END: f32 = PI * 2.25;
/// Pixels of vertical drag for the full range, and with Shift.
const DRAG_PIXELS: f32 = 220.0;
const DRAG_PIXELS_FINE: f32 = 1400.0;
/// Mouse-wheel steps, and with Shift.
const WHEEL_STEP: f32 = 0.02;
const WHEEL_STEP_FINE: f32 = 0.004;

/// How many straight pieces draw an arc of `span` (radians) at `radius`
/// (points) round to the eye: none strays from the circle by more than a
/// tenth of a pixel at `pixels_per_point` (a piece of angle θ strays by
/// about r·θ²/8). Fewer pieces are less to tessellate.
pub fn arc_pieces(span: f32, radius: f32, pixels_per_point: f32) -> usize {
    let r = (radius * pixels_per_point).max(1.0);
    let piece = (8.0 * 0.1 / r).sqrt();
    (span.abs() / piece).ceil().clamp(2.0, 256.0) as usize
}

fn arc(painter: &Painter, center: Pos2, radius: f32, (from, to): (f32, f32), width: f32, color: Color32) {
    let n = arc_pieces(to - from, radius, painter.ctx().pixels_per_point());
    let points: Vec<Pos2> = (0..=n)
        .map(|i| from + (to - from) * i as f32 / n as f32)
        .map(|a| center + radius * vec2(a.cos(), a.sin()))
        .collect();
    // round caps
    painter.circle_filled(points[0], width / 2.0, color);
    painter.circle_filled(points[n], width / 2.0, color);
    painter.add(Shape::line(points, Stroke::new(width, color)));
}

/// How far the mouse wheel turned this frame, in points (as egui's
/// `raw_scroll_delta` was until 0.34): its events summed, lines at 40
/// points (egui's own speed), pages at the window's height. Unsmoothed, so
/// a knob steps once a notch.
pub fn wheel(input: &egui::InputState) -> Vec2 {
    let page = input.viewport_rect().height();
    input.raw.events.iter().fold(Vec2::ZERO, |sum, event| match event {
        egui::Event::MouseWheel { unit, delta, .. } => {
            sum + match unit {
                egui::MouseWheelUnit::Point => *delta,
                egui::MouseWheelUnit::Line => *delta * 40.0,
                egui::MouseWheelUnit::Page => *delta * page,
            }
        }
        _ => sum,
    })
}

/// A host's parameter as the family's widgets drive it: where it is along
/// its travel (0 to 1), its default there, its text at a place, its steps,
/// and gestures (`begin`, `set`, `end`: one undo step for the host).
pub trait HostParam {
    /// What names its widgets among a window's (egui ids).
    fn key(&self) -> String;
    fn position(&self) -> f32;
    fn default_position(&self) -> f32;
    /// Its value at `position`, as the host shows it, with its unit.
    fn text_at(&self, position: f32) -> String;
    /// The place of the step before or after `position` (a stepped one).
    fn step_from(&self, position: f32, up: bool) -> f32;
    fn begin(&self);
    fn set(&self, position: f32);
    fn end(&self);
}

/// `param` set to `position` in one gesture.
fn set_once(param: &impl HostParam, position: f32) {
    param.begin();
    param.set(position);
    param.end();
}

/// What a knob did this frame.
pub struct Turn {
    pub response: Response,
    /// The new position (0..1), if it moved.
    pub value: Option<f32>,
    /// `value` comes from a drag, which `began` and will have `ended`
    /// (one gesture, one undo step for hosts); otherwise it's a single
    /// step (a reset, a wheel click).
    pub dragging: bool,
    pub began: bool,
    pub ended: bool,
}

/// The family knob on a plain 0..1 position, in a `KNOB_CELL`-sized `cell`
/// (`knob_cell`; or a taller one, `knob_cell_for`, for a larger knob),
/// with `text` (the value at a position) and `label` below. `lit` false
/// draws it dimmed. The caller keeps the position: it passes it in each
/// frame and applies `Turn::value`.
///
/// Drag vertically (220 px for the full range, 1400 px with Shift; a finger
/// sideways too, right raising it), use the wheel (2 % a step, 0.4 % with
/// Shift), or reset it to `default` with a double-click, right-click or
/// Ctrl/Cmd-click.
#[allow(clippy::too_many_arguments)]
pub fn knob_at(
    ui: &mut Ui,
    cell: Rect,
    id: Id,
    value: f32,
    default: f32,
    text: impl Fn(f32) -> String,
    label: &str,
    lit: bool,
) -> Turn {
    // (as big as the cell leaves over the two lines, in whole points: `KNOB`
    // in a `knob_cell`, wherever the cell's edges fall)
    let line = knob_line();
    let size = (cell.height() - 4.0 - 2.0 * line).round().max(KNOB);
    let knob_rect = Rect::from_center_size(pos2(cell.center().x, cell.top() + size / 2.0), Vec2::splat(size));
    let response = ui.interact(knob_rect, id, Sense::click_and_drag());
    let mods = ui.input(|i| i.modifiers);
    let mut turn = Turn { response, value: None, dragging: false, began: false, ended: false };
    let response = &turn.response;

    // (by a finger of its own, or egui's pointer: `fingers`)
    let drag = crate::fingers::drag(ui, response);
    if response.double_clicked() || response.secondary_clicked() || (response.clicked() && mods.command) {
        turn.value = Some(default);
    } else {
        if drag.started {
            turn.began = true;
            ui.data_mut(|d| d.insert_temp(id, value));
        }
        // (a finger, as it moves, and its last move as it's lifted; the
        // pointer each frame it drags, as ever)
        let moved = if drag.finger { drag.delta != Vec2::ZERO } else { drag.dragging };
        if moved {
            let pixels = if mods.shift { DRAG_PIXELS_FINE } else { DRAG_PIXELS };
            let start = ui.data(|d| d.get_temp::<f32>(id)).unwrap_or(value);
            let v = (start + drag.rise() / pixels).clamp(0.0, 1.0);
            ui.data_mut(|d| d.insert_temp(id, v));
            turn.value = Some(v);
            turn.dragging = true;
        }
        if drag.stopped {
            turn.ended = true;
        }
    }
    if response.hovered() || drag.dragging {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        let wheel = ui.input(|i| wheel(i).y);
        if wheel != 0.0 && !drag.dragging {
            let step = if mods.shift { WHEEL_STEP_FINE } else { WHEEL_STEP };
            turn.value = Some((value + step * wheel.signum()).clamp(0.0, 1.0));
        }
    }
    let value = turn.value.unwrap_or(value);

    // (a larger knob, an iPad's macro, has a broader ring and tick, by the
    // root of how much larger, and a little larger text in its lines: all
    // as they are at `KNOB`)
    let grow = (size / KNOB).sqrt();
    let ring = RING_WIDTH * grow;
    let painter = ui.painter();
    let center = knob_rect.center();
    let radius = size / 2.0 - ring;
    let accent = if lit { ACCENT() } else { RING() };
    painter.circle_filled(center, radius - ring / 2.0, KNOB_WELL());
    arc(painter, center, radius, (ARC_START, ARC_END), ring, RING());
    let angle = ARC_START + (ARC_END - ARC_START) * value;
    if value > 1e-4 && lit {
        arc(painter, center, radius, (ARC_START, angle), ring, accent);
    }
    let dir = vec2(angle.cos(), angle.sin());
    let (inner, outer) = (radius - ring * 1.8, radius - ring * 0.4);
    let tick = Stroke::new(2.0 * grow, if lit { ACCENT() } else { FAINT() });
    painter.line_segment([center + inner * dir, center + outer * dir], tick);

    let (value_size, label_size) = touch::pick((11.0, 10.0), (13.0, 12.0));
    let (value_size, label_size) = ((value_size * grow.sqrt()).round(), (label_size * grow.sqrt()).round());
    let value_y = knob_rect.bottom() + 4.0 + line / 2.0;
    let color = if lit { VALUE() } else { FAINT() };
    painter.text(pos2(cell.center().x, value_y), Align2::CENTER_CENTER, text(value), font(value_size), color);
    let label = fit(ui, label, &font(label_size), cell.width());
    let label_color = if lit { LABEL() } else { FAINT() };
    let at = pos2(cell.center().x, value_y + line);
    ui.painter().text(at, Align2::CENTER_CENTER, label, font(label_size), label_color);
    turn
}

/// A knob for `param` in a `KNOB_CELL`-sized `cell`, with its value and
/// `label` below (`knob_at`); each drag is one gesture for the host.
pub fn knob(ui: &mut Ui, cell: Rect, param: &impl HostParam, label: &str, lit: bool) -> Response {
    let id = Id::new(("oroboro-knob", param.key()));
    let value = param.position();
    let text = |v| param.text_at(v);
    let turn = knob_at(ui, cell, id, value, param.default_position(), text, label, lit);
    if turn.began {
        param.begin();
    }
    match turn.value {
        Some(v) if turn.dragging => param.set(v),
        Some(v) => set_once(param, v),
        None => {}
    }
    if turn.ended {
        param.end();
    }
    turn.response
}

/// A pill toggle for an on/off `param` (on past the middle of its
/// travel): an LED and `label`.
pub fn toggle(ui: &mut Ui, rect: Rect, param: &impl HostParam, label: &str) -> Response {
    let response = ui.interact(rect, Id::new(("oroboro-toggle", param.key())), Sense::click());
    let mut on = param.position() > 0.5;
    if response.clicked() {
        on = !on;
        set_once(param, if on { 1.0 } else { 0.0 });
    }
    let painter = ui.painter();
    let fill = if response.hovered() { ROW_HOVER() } else { BUTTON() };
    let border = if on { accent_alpha(0x80) } else { BUTTON_BORDER() };
    // (a pill however tall: its ends half circles)
    let round = CornerRadius::same((rect.height() / 2.0).min(255.0) as u8);
    painter.rect_filled(rect, round, fill);
    painter.rect_stroke(rect, round, Stroke::new(1.0, border), StrokeKind::Inside);
    let (led, led_r, text_at, size) = touch::pick((14.0, 3.5, 26.0, 10.0), (18.0, 4.5, 32.0, 13.0));
    painter.circle_filled(pos2(rect.left() + led, rect.center().y), led_r, if on { ACCENT() } else { RING() });
    let text_color = if on { ACCENT_SOFT() } else { SECONDARY() };
    painter.text(pos2(rect.left() + text_at, rect.center().y), Align2::LEFT_CENTER, label, font(size), text_color);
    response
}

/// A recessed step picker for a stepped `param`: its value in accent, with
/// ‹ › halves that step it (the wheel too).
pub fn step_picker(ui: &mut Ui, rect: Rect, param: &impl HostParam) -> Response {
    let response = ui.interact(rect, Id::new(("oroboro-step", param.key())), Sense::click());
    let mut value = param.position();
    let mut target = None;
    if response.clicked() {
        if let Some(p) = response.interact_pointer_pos() {
            target = Some(param.step_from(value, p.x >= rect.center().x));
        }
    }
    if response.hovered() {
        let wheel = ui.input(|i| wheel(i).y);
        if wheel != 0.0 {
            target = Some(param.step_from(value, wheel > 0.0));
        }
    }
    if let Some(t) = target {
        value = t;
        set_once(param, value);
    }
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4), WELL());
    painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, LIST_BORDER()), StrokeKind::Inside);
    let hover_x = response.hover_pos().map(|p| p.x);
    let arrow = |left: bool| {
        let lit = hover_x.is_some_and(|x| (x < rect.center().x) == left);
        if lit {
            TEXT()
        } else {
            FAINT()
        }
    };
    let y = rect.center().y;
    let (inset, arrows, size) = touch::pick((9.0, 13.0, 13.0), (16.0, 18.0, 15.0));
    painter.text(pos2(rect.left() + inset, y), Align2::CENTER_CENTER, "‹", font(arrows), arrow(true));
    painter.text(pos2(rect.right() - inset, y), Align2::CENTER_CENTER, "›", font(arrows), arrow(false));
    let text = param.text_at(value);
    painter.text(rect.center(), Align2::CENTER_CENTER, text, font(size), ACCENT());
    response
}

/// Where a button sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    /// In a section or card.
    Panel,
    /// In the preset bar.
    Bar,
    /// A card's main action: green border, soft-green label.
    Primary,
}

/// A family button showing `text`; `down` shows it pressed (e.g. the preset
/// name while the browser is open). `salt` keeps its id apart from other
/// buttons with the same text.
pub fn button(ui: &mut Ui, rect: Rect, text: &str, kind: ButtonKind, down: bool, salt: impl Hash) -> Response {
    let response = ui.interact(rect, Id::new(("oroboro-button", salt)), Sense::click());
    let painter = ui.painter();
    let (mut fill, mut border) = match kind {
        ButtonKind::Bar => (BUTTON_BAR(), BUTTON_BORDER()),
        ButtonKind::Panel => (BUTTON(), BUTTON_BORDER()),
        ButtonKind::Primary => (BUTTON(), accent_alpha(0x80)),
    };
    if response.hovered() {
        fill = BUTTON_HOVER();
        if kind != ButtonKind::Primary {
            border = BUTTON_HOVER_BORDER();
        }
    }
    if down || response.is_pointer_button_down_on() {
        border = ACCENT();
    }
    painter.rect_filled(rect, CornerRadius::same(4), fill);
    painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, border), StrokeKind::Inside);
    let color = if kind == ButtonKind::Primary { ACCENT_SOFT() } else { TEXT_BUTTON() };
    painter.text(rect.center(), Align2::CENTER_CENTER, text, font(button_text()), color);
    response
}

/// A button's text size: larger with fingers, whose buttons are taller.
pub fn button_text() -> f32 {
    touch::pick(11.0, 14.0)
}

/// A button that opens a list: a panel button with `text` and a small
/// triangle pointing down at its right (drawn: the font has none).
pub fn list_button(ui: &mut Ui, rect: Rect, text: &str, salt: impl Hash) -> Response {
    let response = button(ui, rect, "", ButtonKind::Panel, false, salt);
    let painter = ui.painter();
    let size = button_text();
    let mark = vec2(size * 0.6, size * 0.35);
    let gap = 6.0;
    let galley = painter.layout_no_wrap(text.to_owned(), font(size), TEXT_BUTTON());
    let width = galley.size().x.min(rect.width() - 16.0 - mark.x - gap);
    let left = rect.center().x - (width + gap + mark.x) / 2.0;
    let text = fit(ui, text, &font(size), width);
    painter.text(pos2(left, rect.center().y), Align2::LEFT_CENTER, text, font(size), TEXT_BUTTON());
    let tip = pos2(left + width + gap + mark.x / 2.0, rect.center().y + mark.y / 2.0);
    let points = vec![tip + vec2(-mark.x / 2.0, -mark.y), tip + vec2(mark.x / 2.0, -mark.y), tip];
    painter.add(Shape::convex_polygon(points, TEXT_BUTTON(), Stroke::NONE));
    response
}

/// A section: the panel with its head (accent mark, UPPERCASE title).
/// Returns the body inside the padding.
pub fn section(ui: &Ui, rect: Rect, title: &str) -> Rect {
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(8), PANEL());
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, PANEL_BORDER()), StrokeKind::Inside);
    head(painter, pos2(rect.left() + 12.0, rect.top() + 10.0), title);
    Rect::from_min_max(pos2(rect.left() + 12.0, rect.top() + section_head()), rect.max - vec2(12.0, 10.0))
}

/// Where a section's body starts under its top: under the head (larger
/// with fingers).
pub fn section_head() -> f32 {
    touch::pick(10.0 + 14.0 + 6.0, 10.0 + 16.0 + 8.0)
}

/// A section head at `top_left`: a 3 × 10 accent mark, 6 px, the title.
pub fn head(painter: &Painter, top_left: Pos2, title: &str) {
    let (y, size) = touch::pick((top_left.y + 7.0, 10.0), (top_left.y + 8.0, 12.0));
    let mark = Rect::from_min_size(pos2(top_left.x, y - 5.0), vec2(3.0, 10.0));
    painter.rect_filled(mark, CornerRadius::same(1), ACCENT());
    painter.text(pos2(top_left.x + 9.0, y), Align2::LEFT_CENTER, title.to_uppercase(), font(size), SECONDARY());
}

/// A small group title (9 pt, 11 with fingers, UPPERCASE) inside a section.
pub fn group_title(painter: &Painter, pos: Pos2, title: &str) {
    painter.text(pos, Align2::LEFT_TOP, title.to_uppercase(), font(touch::pick(9.0, 11.0)), MUTED());
}

/// The 1 px rule between groups.
pub fn divider(painter: &Painter, x: f32, top: f32, bottom: f32) {
    painter.line_segment([pos2(x, top), pos2(x, bottom)], Stroke::new(1.0, DIVIDER()));
}

/// The brand: "OROBORO" in accent and the product's name, 20 pt. A
/// borderless button (it opens the About panel).
pub fn brand(ui: &mut Ui, rect: Rect, product: &str) -> Response {
    let response = ui.interact(rect, Id::new("oroboro-brand"), Sense::click());
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(rect.expand2(vec2(6.0, 0.0)), CornerRadius::same(4), BRAND_HOVER());
    }
    let y = rect.center().y;
    let first = painter.text(pos2(rect.left(), y), Align2::LEFT_CENTER, "OROBORO", font(20.0), ACCENT());
    painter.text(pos2(first.right() + 6.0, y), Align2::LEFT_CENTER, product, font(20.0), BRAND_SECOND());
    response
}

/// A modal overlay over `area`: dims it and takes every click, so what's
/// behind can't be touched. `contents` draws on top of it.
pub fn overlay<R>(ctx: &egui::Context, name: &str, area: Rect, contents: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Area::new(Id::new(("oroboro-overlay", name)))
        .order(egui::Order::Foreground)
        .fixed_pos(area.min)
        .show(ctx, |ui| {
            ui.allocate_rect(area, Sense::click_and_drag());
            // (dimmed past the safe area's edges it reaches, to the screen's:
            // iOS's strips; not over what it leaves, as the browser the title bar)
            let (content, screen) = (ctx.content_rect(), ctx.viewport_rect());
            let dimmed = Rect::from_min_max(
                pos2(
                    if area.left() <= content.left() { screen.left() } else { area.left() },
                    if area.top() <= content.top() { screen.top() } else { area.top() },
                ),
                pos2(
                    if area.right() >= content.right() { screen.right() } else { area.right() },
                    if area.bottom() >= content.bottom() { screen.bottom() } else { area.bottom() },
                ),
            );
            let mut dim = ui.painter().clone();
            dim.set_clip_rect(dimmed);
            dim.rect_filled(dimmed, CornerRadius::ZERO, overlay_dim());
            contents(ui)
        })
        .inner
}

/// Where a card of `size` goes in `area` (an overlay's): in its middle;
/// or, when it's taller or wider than the area (a phone's screen), moved by
/// dragging it, or the wheel, so all of it can be reached (what's on it
/// takes the pointer first): it starts with its top left in view, and a
/// thin bar at its edge shows how much of it is (as iOS's scroll
/// indicators). `salt` keeps how far it's moved.
pub fn card_rect(ui: &Ui, area: Rect, size: Vec2, salt: impl Hash) -> Rect {
    let room = area.shrink(8.0);
    let middle = Rect::from_center_size(area.center(), size);
    if size.x <= room.width() && size.y <= room.height() {
        return middle;
    }
    // (at most till its far edge is in the room)
    let slack = ((size - room.size()) / 2.0).max(Vec2::ZERO);
    let id = Id::new(("oroboro-card-moved", salt));
    let response = ui.interact(area, id, Sense::drag());
    let mut moved: Vec2 = ui.data(|d| d.get_temp(id)).unwrap_or(slack);
    moved += response.drag_delta();
    if response.hovered() {
        moved += ui.input(|i| i.smooth_scroll_delta);
    }
    let moved = moved.clamp(-slack, slack);
    ui.data_mut(|d| d.insert_temp(id, moved));
    let card = middle.translate(moved);
    // the bar: over the card (drawn after this), along its right edge
    if size.y > room.height() {
        let shown = room.height() / size.y;
        let length = (room.height() * shown).max(24.0);
        let done = (room.top() - card.top()) / (size.y - room.height());
        let top = room.top() + done * (room.height() - length);
        let bar = Rect::from_min_size(pos2(card.right() - 7.0, top), vec2(3.0, length));
        let painter = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, id));
        painter.rect_filled(bar, CornerRadius::same(2), MUTED());
    }
    card
}

/// A sheet (with fingers, an overlay as big as the window, its title left
/// out): shadow and panel. Returns the body inside its padding (12).
pub fn sheet(ui: &Ui, rect: Rect) -> Rect {
    let painter = ui.painter();
    let shadow = epaint::Shadow { offset: [0, 12], blur: 40, spread: 0, color: SHADOW() };
    painter.add(shadow.as_shape(rect, CornerRadius::same(8)));
    painter.rect_filled(rect, CornerRadius::same(8), PANEL());
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, PANEL_BORDER()), StrokeKind::Inside);
    rect.shrink(12.0)
}

/// An overlay card: shadow, panel, the section-style head with `heading`.
/// Returns the body inside the padding (20 top and bottom, 24 at the sides).
pub fn card(ui: &Ui, rect: Rect, heading: &str) -> Rect {
    let painter = ui.painter();
    let shadow = epaint::Shadow { offset: [0, 20], blur: 60, spread: 0, color: SHADOW() };
    painter.add(shadow.as_shape(rect, CornerRadius::same(8)));
    painter.rect_filled(rect, CornerRadius::same(8), PANEL());
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, PANEL_BORDER()), StrokeKind::Inside);
    head(painter, pos2(rect.left() + 24.0, rect.top() + 20.0), heading);
    Rect::from_min_max(pos2(rect.left() + 24.0, rect.top() + 20.0 + 14.0 + 12.0), rect.max - vec2(24.0, 20.0))
}

/// `text` cut at the end with "…" until it's at most `width` wide in `font`.
pub fn fit(ui: &Ui, text: &str, font: &FontId, width: f32) -> String {
    let wide = |t: String| ui.ctx().fonts_mut(|f| f.layout_no_wrap(t, font.clone(), Color32::WHITE).size().x);
    if wide(text.to_string()) <= width {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let mut keep = chars.len();
    while keep > 0 {
        keep -= 1;
        let cut = format!("{}…", chars[..keep].iter().collect::<String>().trim_end());
        if wide(cut.clone()) <= width {
            return cut;
        }
    }
    "…".to_string()
}

/// How wide the family's font sets each character from the space to `~`,
/// per point of font size (measured at 100 points, where setting glyphs on
/// whole pixels no longer shows).
#[rustfmt::skip]
const ADVANCES: [f32; 95] = [
    0.2597, 0.2456, 0.3634, 0.6450, 0.5703, 0.8109, 0.7159, 0.1988, 0.2775, 0.2775,
    0.5503, 0.5703, 0.2256, 0.3216, 0.2438, 0.3466, 0.5703, 0.5703, 0.5703, 0.5703,
    0.5703, 0.5703, 0.5703, 0.5703, 0.5703, 0.5703, 0.2438, 0.2438, 0.5703, 0.5703,
    0.5703, 0.4234, 0.8878, 0.6072, 0.6309, 0.6222, 0.7100, 0.5522, 0.5053, 0.7228,
    0.7228, 0.3106, 0.2516, 0.5881, 0.5144, 0.8706, 0.7278, 0.7669, 0.5881, 0.7669,
    0.5991, 0.5453, 0.5303, 0.7209, 0.5813, 0.8988, 0.5472, 0.5353, 0.5741, 0.3206,
    0.3466, 0.3206, 0.5703, 0.4175, 0.2647, 0.5353, 0.5981, 0.4753, 0.5981, 0.5512,
    0.3084, 0.5981, 0.5950, 0.2328, 0.2328, 0.4931, 0.2328, 0.8928, 0.5950, 0.5891,
    0.5981, 0.5981, 0.3934, 0.4684, 0.3394, 0.5950, 0.4684, 0.7350, 0.5031, 0.4684,
    0.4622, 0.3584, 0.5422, 0.3584, 0.5703,
];
/// What a character outside that range counts as: a wide one.
const OTHER_ADVANCE: f32 = 0.9;

/// About how wide `text` is at `size` points in the family's font, without
/// a font at hand: for laying out where there is no `Ui` (how many rows of
/// controls a panel needs). The font sets each glyph on a whole pixel, so
/// the width drawn differs by up to half a pixel a letter either way; draw
/// with `fitting`, which makes up for that.
pub fn text_width(text: &str, size: f32) -> f32 {
    let advance = |c: char| match u32::from(c).checked_sub(32).and_then(|i| ADVANCES.get(i as usize)) {
        Some(a) => *a,
        None => OTHER_ADVANCE,
    };
    text.chars().map(advance).sum::<f32>() * size
}

/// `text` and the font size to draw it in so that it's at most `width`
/// wide: at `size` if it fits, else smaller, down to `least` of it (0..1),
/// and only then cut with "…" (`fit`).
pub fn fitting(ui: &Ui, text: &str, size: f32, width: f32, least: f32) -> (String, f32) {
    let wide = |s: f32| ui.ctx().fonts_mut(|f| f.layout_no_wrap(text.to_string(), FontId::proportional(s), Color32::WHITE).size().x);
    let mut at = size;
    for _ in 0..8 {
        let w = wide(at);
        if w <= width {
            return (text.to_string(), at);
        }
        // (glyphs sit on whole pixels: a size a little under what the
        // widths' ratio says, till it fits)
        let next = (at * (width / w).min(0.97)).max(size * least);
        if next >= at {
            break;
        }
        at = next;
    }
    if wide(at) <= width {
        return (text.to_string(), at);
    }
    (fit(ui, text, &FontId::proportional(at), width), at)
}

/// `text` cut to at most `max` characters from the start, with "…" (long
/// folder paths keep their end).
pub fn elide_start(text: &str, max: usize) -> String {
    let n = text.chars().count();
    if n <= max {
        text.to_string()
    } else {
        let tail: String = text.chars().skip(n - max + 1).collect();
        format!("…{tail}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elides_long_paths_at_the_start() {
        assert_eq!(elide_start("C:\\Patches", 20), "C:\\Patches");
        assert_eq!(elide_start("abcdefgh", 5), "…efgh");
    }

    /// The width worked out without a font is the font's, for every
    /// character it's kept for and for words; and a text that doesn't fit
    /// gets smaller before it's cut.
    #[test]
    fn widths_without_a_font_are_the_fonts_and_long_texts_shrink_before_theyre_cut() {
        let ctx = egui::Context::default();
        crate::theme::install(&ctx);
        let _ = ctx.run_ui(Default::default(), |ui| {
            let fonts = ui.ctx().clone();
            let real = |t: &str, size: f32| {
                fonts.fonts_mut(|f| f.layout_no_wrap(t.to_string(), FontId::proportional(size), Color32::WHITE).size().x)
            };
            for c in (32u8..127).map(char::from) {
                let (t, size) = (c.to_string(), 100.0);
                assert!((text_width(&t, size) - real(&t, size)).abs() < 0.6, "{c:?}: {}", real(&t, size));
            }
            for word in ["Release", "Transpose", "Ping-pong", "1.25 kHz", "V/Oct", "S&H"] {
                let (ours, font) = (text_width(word, 100.0), real(word, 100.0));
                assert!((ours - font).abs() < 0.012 * font, "{word}: {ours} for {font}");
                // at a label's size the glyphs' whole pixels show: within
                // half a pixel a letter
                let (ours, font) = (text_width(word, 8.5), real(word, 8.5));
                assert!((ours - font).abs() <= 0.5 * word.chars().count() as f32, "{word}: {ours} for {font}");
            }
            assert_eq!(text_width("", 12.0), 0.0);
            assert!(text_width("é", 10.0) >= real("é", 10.0), "a letter it doesn't know counts as a wide one");

            egui::CentralPanel::default().show_inside(ui, |ui| {
                // it fits: as it is
                let wide = real("Feedback", 9.0);
                assert_eq!(fitting(ui, "Feedback", 9.0, wide + 0.1, 0.7), ("Feedback".to_string(), 9.0));
                // a little too wide: smaller, whole
                let (text, size) = fitting(ui, "Feedback", 9.0, wide * 0.85, 0.7);
                assert_eq!(text, "Feedback");
                assert!((0.7 * 9.0..9.0).contains(&size) && real("Feedback", size) <= wide * 0.85, "{size}");
                // far too wide: as small as it goes, then cut
                let (text, size) = fitting(ui, "Feedback", 9.0, wide * 0.4, 0.7);
                assert!((size - 0.7 * 9.0).abs() < 1e-4 && text.ends_with('…') && text.starts_with('F'), "{text} {size}");
                assert!(real(&text, size) <= wide * 0.4);
            });
        });
    }
}
