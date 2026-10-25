//! Fingers on controls, each its own. egui has one pointer, which the
//! first finger down drives (as the iOS and Windows editors make it: every
//! finger is an `egui::Event::Touch`, the first the pointer too), so a knob
//! a second finger turned showed the press and never moved, while the first
//! played a key. Here a finger that comes down on a control is that
//! control's till it's lifted: a control asks `drag(ui, &response)` in
//! place of its response's drag, and gets its own finger's moves (with the
//! feel the pointer's drag has, the control's own: it only swaps where the
//! moves come from), or else the pointer's as ever, so a mouse works as it
//! did. The first finger, the pointer too, on a control counts once: the
//! control takes the finger and passes over the pointer. Several controls
//! may each hold a finger at once.
//!
//! Which control a finger came down on is known as the frame it comes in
//! starts, from where the controls were the frame before (as egui finds
//! what a press is on): every control that asks `drag` is a place a finger
//! can land, the one drawn last on top; the on-screen keyboard, which reads
//! every finger itself, is one for as many fingers as land on it (`hold`).
//! So the patch canvas can tell fingers no control holds (`all_loose`: a
//! pinch is theirs) from a finger turning a knob or playing a key.
//!
//! A finger's contact lands a little off what it aims at (a real one on a
//! Surface's screen came down 4 to 12 points above a knob's dial), so a
//! control takes a finger beside it too, the nearest within its reach
//! (`reach`: a third of its smaller side, 5 to 18 points); the keyboard
//! only those on it. Taken from beside, the finger egui's pointer follows
//! makes its drag the control's (`set_dragged_id`), so nothing the pointer
//! came down on (a scroll area) moves with it.

use egui::{Context, Id, LayerId, Pos2, Rect, Response, TouchId, TouchPhase, Ui, Vec2};

/// What a control's drag did this frame: by a finger of its own, else by
/// egui's pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Drag {
    /// It began this frame.
    pub started: bool,
    /// It's on (began, moving or held still).
    pub dragging: bool,
    /// It ended this frame (lifted, cancelled).
    pub stopped: bool,
    /// How far it moved this frame.
    pub delta: Vec2,
    /// Where it is (the finger, or the pointer's press).
    pub at: Option<Pos2>,
    /// It's a finger of the control's own (not egui's pointer).
    pub finger: bool,
}

impl Drag {
    /// How far it raised a knob this frame, in points: up, and with a finger
    /// right too (on a touch screen a finger turns a knob sideways as
    /// readily; a mouse up and down only).
    pub fn rise(&self) -> f32 {
        if self.finger {
            self.delta.x - self.delta.y
        } else {
            -self.delta.y
        }
    }
}

/// A finger down, or lifted this frame.
#[derive(Clone, Debug)]
struct Finger {
    id: TouchId,
    at: Pos2,
    /// How far it moved this frame.
    moved: Vec2,
    /// The frame it came down in (egui's pass).
    since: u64,
    /// The control (or keyboard) it came down on, which holds it.
    owner: Option<Id>,
    /// Lifted (or cancelled) this frame.
    ended: bool,
    /// The finger egui's pointer follows (the first down while no other
    /// was), and whether it came down beside its control, not on it.
    primary: bool,
    beside: bool,
}

/// A place a finger can land: a control (one finger) or the keyboard (any).
#[derive(Clone, Copy, Debug)]
struct Place {
    id: Id,
    rect: Rect,
    layer: LayerId,
    many: bool,
}

/// The fingers and the places, kept in the context's memory.
#[derive(Clone, Debug, Default)]
struct Fingers {
    /// The pass the fingers were last brought up to.
    pass: Option<u64>,
    all: Vec<Finger>,
    /// The places of this frame (the next frame's fingers land on them) and
    /// of the frame before (this frame's land on them).
    now: Vec<Place>,
    before: Vec<Place>,
}

/// A finger's event this frame, with the layer on top where it came down.
type Seen = (TouchId, TouchPhase, Pos2, Option<LayerId>);

impl Fingers {
    /// Into pass `pass` with its touches: last frame's lifted fingers gone,
    /// new ones on the places they came down on.
    fn advance(&mut self, pass: u64, events: &[Seen]) {
        self.all.retain(|f| !f.ended);
        for f in &mut self.all {
            f.moved = Vec2::ZERO;
        }
        self.before = std::mem::take(&mut self.now);
        for &(id, phase, at, layer) in events {
            match phase {
                TouchPhase::Start => {
                    // (one down again without its lift: that one goes)
                    self.all.retain(|f| f.id != id);
                    let primary = self.all.iter().all(|f| f.ended);
                    let owner = self.place_at(at, layer);
                    let beside = owner.is_some_and(|o| !self.before.iter().any(|p| p.id == o && p.rect.contains(at)));
                    self.all.push(Finger { id, at, moved: Vec2::ZERO, since: pass, owner, ended: false, primary, beside });
                }
                TouchPhase::Move => {
                    if let Some(f) = self.all.iter_mut().find(|f| f.id == id && !f.ended) {
                        f.moved += at - f.at;
                        f.at = at;
                    }
                }
                TouchPhase::End | TouchPhase::Cancel => {
                    if let Some(f) = self.all.iter_mut().find(|f| f.id == id && !f.ended) {
                        f.moved += at - f.at;
                        f.at = at;
                        f.ended = true;
                    }
                }
            }
        }
        self.pass = Some(pass);
    }

    /// The place a finger coming down at `at` (on `layer`, the one on top
    /// there) lands on: of those on that layer that can take another finger,
    /// the one it's on, else the nearest within its reach (`reach`; the
    /// keyboard none), the last drawn where two are as near.
    fn place_at(&self, at: Pos2, layer: Option<LayerId>) -> Option<Id> {
        let mut best: Option<(f32, Id)> = None;
        for p in self.before.iter().rev() {
            let free = p.many || !self.all.iter().any(|f| f.owner == Some(p.id) && !f.ended);
            if !free || !layer.is_none_or(|l| l == p.layer) {
                continue;
            }
            let off = p.rect.distance_to_pos(at);
            if off <= if p.many { 0.0 } else { reach(p.rect) } && best.is_none_or(|(d, _)| off < d) {
                best = Some((off, p.id));
            }
        }
        best.map(|(_, id)| id)
    }
}

/// How far beside a control a finger may come down and take it: a third of
/// its smaller side, from egui's own reach for the pointer (5 points) to 18.
fn reach(rect: Rect) -> f32 {
    (rect.size().min_elem() / 3.0).clamp(5.0, 18.0)
}

/// The context's fingers, brought up to this frame (the first ask in a
/// frame reads its touches), for `f`.
fn with<R>(ctx: &Context, f: impl FnOnce(&mut Fingers, u64) -> R) -> R {
    let key = Id::new("oroboro-fingers");
    let pass = ctx.cumulative_pass_nr();
    let fresh = ctx.data(|d| d.get_temp::<Option<u64>>(key.with("pass"))).flatten() != Some(pass);
    if fresh {
        let touches: Vec<(TouchId, TouchPhase, Pos2)> = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|e| match e {
                    egui::Event::Touch { id, phase, pos, .. } => Some((*id, *phase, *pos)),
                    _ => None,
                })
                .collect()
        });
        // (the layer on top where each comes down, asked outside the memory's lock)
        let seen: Vec<Seen> = touches
            .into_iter()
            .map(|(id, phase, at)| (id, phase, at, (phase == TouchPhase::Start).then(|| ctx.layer_id_at(at)).flatten()))
            .collect();
        ctx.data_mut(|d| {
            d.get_temp_mut_or_default::<Fingers>(key).advance(pass, &seen);
            d.insert_temp(key.with("pass"), Some(pass));
        });
    }
    ctx.data_mut(|d| f(d.get_temp_mut_or_default::<Fingers>(key), pass))
}

/// The drag of the control `response` is (egui's `Sense::drag`): a finger
/// of its own, that came down on it, while it's down; else egui's pointer,
/// as `response` has it. The control's place is kept for the next frame's
/// fingers. With a mouse (no fingers) it's the response's drag exactly.
pub fn drag(ui: &Ui, response: &Response) -> Drag {
    let place = Place { id: response.id, rect: response.interact_rect, layer: response.layer_id, many: false };
    let finger = with(ui.ctx(), |fingers, pass| {
        fingers.now.push(place);
        fingers.all.iter().find(|f| f.owner == Some(response.id)).map(|f| {
            (f.since == pass, f.ended, f.moved, f.at, f.primary && f.beside)
        })
    });
    if let Some((started, ended, delta, at, pointer_beside)) = finger {
        if started && !ended && pointer_beside {
            ui.ctx().set_dragged_id(response.id);
        }
        return Drag { started, dragging: !ended, stopped: ended, delta, at: Some(at), finger: true };
    }
    let dragging = response.dragged();
    Drag {
        started: response.drag_started(),
        dragging,
        stopped: response.drag_stopped(),
        delta: if dragging { response.drag_delta() } else { Vec2::ZERO },
        at: response.interact_pointer_pos(),
        finger: false,
    }
}

/// `rect` (of `ui`'s layer) takes every finger that comes down on it, as
/// `id`'s (the on-screen keyboard, which reads them itself): no control
/// under it takes them, and they aren't loose (`all_loose`).
pub fn hold(ui: &Ui, id: Id, rect: Rect) {
    with(ui.ctx(), |fingers, _| fingers.now.push(Place { id, rect, layer: ui.layer_id(), many: true }));
}

/// Whether two or more fingers are down and none is held (by a control or
/// the keyboard): a pinch, a two-finger move, are theirs.
pub fn all_loose(ctx: &Context) -> bool {
    with(ctx, |fingers, _| {
        let down: Vec<&Finger> = fingers.all.iter().filter(|f| !f.ended).collect();
        down.len() >= 2 && down.iter().all(|f| f.owner.is_none())
    })
}

/// Whether a finger is down on a control or the keyboard (a control being
/// turned that egui's pointer doesn't know of).
pub fn any_held(ctx: &Context) -> bool {
    with(ctx, |fingers, _| fingers.all.iter().any(|f| f.owner.is_some() && !f.ended))
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2, Sense};

    fn touch(id: u64, phase: TouchPhase, pos: Pos2) -> egui::Event {
        egui::Event::Touch { device_id: egui::TouchDeviceId(0), id: TouchId(id), phase, pos, force: None }
    }

    /// The first finger down drives the pointer too, as the editors make it.
    fn pointer(pos: Pos2, pressed: Option<bool>) -> Vec<egui::Event> {
        let mut events = vec![egui::Event::PointerMoved(pos)];
        if let Some(pressed) = pressed {
            let button = egui::PointerButton::Primary;
            events.push(egui::Event::PointerButton { pos, button, pressed, modifiers: Default::default() });
        }
        events
    }

    /// A frame of two drag controls, `a` and `b`: what each one's `drag`
    /// said.
    fn frame(ctx: &Context, events: Vec<egui::Event>, time: f64) -> (Drag, Drag, bool) {
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(400.0, 300.0));
        let input = egui::RawInput { screen_rect: Some(screen), time: Some(time), events, ..Default::default() };
        let mut said = (Drag::default(), Drag::default(), false);
        let _ = ctx.run_ui(input, |ui| {
            let a = ui.interact(A, Id::new("a"), Sense::click_and_drag());
            let b = ui.interact(B, Id::new("b"), Sense::click_and_drag());
            said = (drag(ui, &a), drag(ui, &b), all_loose(ui.ctx()));
        });
        said
    }

    const A: Rect = Rect { min: pos2(20.0, 20.0), max: pos2(60.0, 60.0) };
    const B: Rect = Rect { min: pos2(120.0, 20.0), max: pos2(160.0, 60.0) };

    /// Two fingers on two controls: each moves its own, by its own finger's
    /// moves; the first finger, which is egui's pointer too, counts once.
    #[test]
    fn two_fingers_hold_two_controls() {
        let ctx = Context::default();
        frame(&ctx, Vec::new(), 0.0);
        let (a, b) = (A.center(), B.center());
        let mut first = vec![touch(1, TouchPhase::Start, a)];
        first.extend(pointer(a, Some(true)));
        let (da, db, _) = frame(&ctx, first, 0.1);
        assert!(da.started && da.finger && !db.dragging, "{da:?} {db:?}");
        let (da, db, loose) = frame(&ctx, vec![touch(2, TouchPhase::Start, b)], 0.2);
        assert!(!da.started && da.dragging && db.started && db.finger, "{da:?} {db:?}");
        assert!(!loose, "both held");
        // both move, each by its own finger (the pointer, finger 1's, too)
        let mut moves = vec![touch(1, TouchPhase::Move, a + vec2(0.0, -30.0)), touch(2, TouchPhase::Move, b + vec2(0.0, 12.0))];
        moves.extend(pointer(a + vec2(0.0, -30.0), None));
        let (da, db, _) = frame(&ctx, moves, 0.3);
        assert_eq!(da.delta, vec2(0.0, -30.0), "once, not twice");
        assert_eq!(db.delta, vec2(0.0, 12.0));
        // one lifted, the other held on
        let (da, db, _) = frame(&ctx, vec![touch(2, TouchPhase::End, b + vec2(0.0, 12.0))], 0.4);
        assert!(da.dragging && !da.stopped && db.stopped && db.finger, "{da:?} {db:?}");
        let (_, db, _) = frame(&ctx, Vec::new(), 0.5);
        assert!(!db.dragging && !db.stopped, "let go");
        let mut up = vec![touch(1, TouchPhase::End, a)];
        up.extend(pointer(a, Some(false)));
        let (da, _, _) = frame(&ctx, up, 0.6);
        assert!(da.stopped);
        assert!(!any_held(&ctx));
    }

    /// With a mouse it's egui's own drag; a control takes one finger, a
    /// second on it is loose; fingers on nothing are loose (a pinch).
    #[test]
    fn a_mouse_drags_as_ever_and_loose_fingers_pinch() {
        let ctx = Context::default();
        frame(&ctx, Vec::new(), 0.0);
        let a = A.center();
        let mut events = pointer(a, Some(true));
        frame(&ctx, std::mem::take(&mut events), 0.1);
        let (da, _, _) = frame(&ctx, pointer(a + vec2(0.0, 20.0), None), 0.2);
        assert!(da.dragging && !da.finger && da.delta == vec2(0.0, 20.0), "{da:?}");
        let (da, _, _) = frame(&ctx, pointer(a + vec2(0.0, 20.0), Some(false)), 0.3);
        assert!(da.stopped && !da.finger, "{da:?}");
        // two fingers on the empty window: loose
        let (p, q) = (pos2(250.0, 200.0), pos2(300.0, 220.0));
        frame(&ctx, vec![touch(5, TouchPhase::Start, p), touch(6, TouchPhase::Start, q)], 0.4);
        let (.., loose) = frame(&ctx, vec![touch(5, TouchPhase::Move, p + vec2(5.0, 0.0))], 0.5);
        assert!(loose);
        frame(&ctx, vec![touch(5, TouchPhase::End, p), touch(6, TouchPhase::End, q)], 0.6);
        // a second finger on a control already held: not the control's
        frame(&ctx, vec![touch(7, TouchPhase::Start, a)], 0.7);
        let (da, ..) = frame(&ctx, vec![touch(8, TouchPhase::Start, a + vec2(5.0, 5.0))], 0.8);
        let (da2, ..) = frame(&ctx, vec![touch(8, TouchPhase::Move, a + vec2(5.0, 40.0))], 0.9);
        assert!(da.dragging && da2.delta == Vec2::ZERO, "the first finger's: {da2:?}");
    }

    /// A finger that comes down a little beside a control (as a real one on
    /// a Surface's screen did, above a knob's dial) takes it, and its drag:
    /// egui's pointer, that finger, drags nothing else; one further off
    /// takes nothing.
    #[test]
    fn a_finger_beside_a_control_takes_it() {
        let ctx = Context::default();
        frame(&ctx, Vec::new(), 0.0);
        // 10 points over A's top edge (its reach: a third of 40)
        let near = pos2(A.center().x, A.top() - 10.0);
        let mut down = vec![touch(1, TouchPhase::Start, near)];
        down.extend(pointer(near, Some(true)));
        let (da, db, _) = frame(&ctx, down, 0.1);
        assert!(da.started && da.finger && !db.dragging, "{da:?}");
        assert_eq!(ctx.dragged_id(), Some(Id::new("a")), "the pointer's drag is A's");
        let mut moves = vec![touch(1, TouchPhase::Move, near - vec2(0.0, 25.0))];
        moves.extend(pointer(near - vec2(0.0, 25.0), None));
        let (da, ..) = frame(&ctx, moves, 0.2);
        assert_eq!(da.delta, vec2(0.0, -25.0));
        let mut up = vec![touch(1, TouchPhase::End, near - vec2(0.0, 25.0))];
        up.extend(pointer(near - vec2(0.0, 25.0), Some(false)));
        let (da, ..) = frame(&ctx, up, 0.3);
        assert!(da.stopped);
        frame(&ctx, vec![egui::Event::PointerGone], 0.4);
        // 20 points off: beyond its reach, nobody's
        let far = pos2(A.center().x, A.top() - 20.0);
        let (da, db, _) = frame(&ctx, vec![touch(2, TouchPhase::Start, far)], 0.5);
        assert!(!da.finger && !db.finger, "{da:?} {db:?}");
        frame(&ctx, vec![touch(2, TouchPhase::End, far)], 0.6);
        assert_eq!(reach(A), 40.0 / 3.0);
        // a finger raises a knob up and to the right; the mouse up only
        let finger = Drag { finger: true, delta: vec2(10.0, -5.0), ..Drag::default() };
        assert_eq!((finger.rise(), Drag { finger: false, ..finger }.rise()), (15.0, 5.0));
        let square = |side: f32| Rect::from_min_size(Pos2::ZERO, vec2(side, side));
        assert_eq!((reach(square(9.0)), reach(square(90.0))), (5.0, 18.0));
    }
}
