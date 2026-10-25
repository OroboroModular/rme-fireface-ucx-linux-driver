//! Hand drawn: a theme's `sketch` redraws a frame's outlines as a hand
//! would, just before they're tessellated, so no drawing code changes and
//! a theme without it looks as it always did. Module borders, cables, knob
//! rings, jacks and lines wobble a little along their length; a
//! rectangle's sides run on past its corners, and a circle or a rounded
//! box closes with a small overlap. Fills, text and pictures are left as
//! they are.
//!
//! The wobble comes from each shape's own form (its size, its points
//! relative to its first), not from where it is, so a patch scrolled or a
//! module moved keeps its lines; only a shape that changes form (a cable
//! being dragged) wobbles anew.

use egui::epaint::{
    CircleShape, ClippedShape, ColorMode, PathShape, PathStroke, RectShape,
    Shape, Stroke, StrokeKind,
};
use egui::{vec2, Color32, Pos2, Rect, Vec2};
use std::f32::consts::TAU;

/// The longest piece of a hand-drawn line (px): longer ones are cut up so
/// they can wobble.
const STEP: f32 = 9.0;
/// How long one wobble is along a line (px).
const WAVE: f32 = 34.0;
/// How far a line strays from its path (px) at roughness 1.
const STRAY: f32 = 1.25;
/// How far a box's sides run past its corners (px) at roughness 1.
const OVERSHOOT: f32 = 3.2;
/// How far a closed outline runs past its start (px) at roughness 1.
const OVERLAP: f32 = 9.0;
/// A box whose corners are rounded more than this (px) is drawn round, in
/// one line; a sharper one side by side.
const ROUND: f32 = 2.5;
/// Boxes and circles are gone over twice, the second time lighter (its
/// share of the colour's opacity), as a pencil goes over a line.
const AGAIN: f32 = 0.5;

/// Redraw every outline in `shapes` as a hand would, `amount` rough (0:
/// as they are; 1: the Sketch theme's).
pub fn roughen(shapes: &mut [ClippedShape], amount: f32) {
    if amount <= 0.0 || !amount.is_finite() {
        return;
    }
    for clipped in shapes {
        let shape = std::mem::replace(&mut clipped.shape, Shape::Noop);
        clipped.shape = rough(shape, amount);
    }
}

fn rough(shape: Shape, amount: f32) -> Shape {
    match shape {
        Shape::Vec(shapes) => Shape::Vec(shapes.into_iter().map(|s| rough(s, amount)).collect()),
        Shape::Rect(r) if shows(r.stroke) && r.brush.is_none() => rect(r, amount),
        Shape::Circle(c) if shows(c.stroke) && c.radius >= 2.0 => circle(c, amount),
        Shape::LineSegment { points, stroke } if shows(stroke) => {
            let seed = form(&points, stroke.color, stroke.width);
            line(wobble(&points, seed, amount), stroke.width, stroke.color)
        }
        Shape::Path(p) => path(p, amount),
        Shape::CubicBezier(b) => match solid(&b.stroke) {
            Some(color) if b.stroke.width > 0.0 && !b.closed => {
                let seed = form(&b.points, color, b.stroke.width);
                let fill = filled(b.fill, b.points.to_vec(), b.closed);
                with_fill(fill, line(wobble(&b.flatten(Some(0.25)), seed, amount), b.stroke.width, color))
            }
            _ => Shape::CubicBezier(b),
        },
        Shape::QuadraticBezier(b) => match solid(&b.stroke) {
            Some(color) if b.stroke.width > 0.0 && !b.closed => {
                let seed = form(&b.points, color, b.stroke.width);
                with_fill(None, line(wobble(&b.flatten(Some(0.25)), seed, amount), b.stroke.width, color))
            }
            _ => Shape::QuadraticBezier(b),
        },
        other => other,
    }
}

fn shows(stroke: Stroke) -> bool {
    stroke.width > 0.0 && stroke.color != Color32::TRANSPARENT
}

fn solid(stroke: &PathStroke) -> Option<Color32> {
    match &stroke.color {
        ColorMode::Solid(c) if *c != Color32::TRANSPARENT => Some(*c),
        _ => None,
    }
}

/// A box: its fill as it was, its outline drawn side by side (sharp
/// corners) or round in one line (rounded ones).
fn rect(r: RectShape, amount: f32) -> Shape {
    let (stroke, kind) = (r.stroke, r.stroke_kind);
    let outline = match kind {
        StrokeKind::Inside => r.rect.shrink(stroke.width / 2.0),
        StrokeKind::Outside => r.rect.expand(stroke.width / 2.0),
        StrokeKind::Middle => r.rect,
    };
    let seed = mix(hash_f(outline.width()) ^ hash_f(outline.height()).rotate_left(21) ^ u64::from(color_bits(stroke.color)));
    let radius = f32::from(r.corner_radius.nw.max(r.corner_radius.ne).max(r.corner_radius.sw).max(r.corner_radius.se));
    let mut fill = r;
    fill.stroke = Stroke::NONE;
    let fill = (fill.fill != Color32::TRANSPARENT).then_some(Shape::Rect(fill));
    let pass = |seed: u64, color: Color32| -> Vec<Shape> {
        if radius > ROUND {
            let points = rounded(outline, radius);
            return vec![line(closing(&points, seed, amount), stroke.width, color)];
        }
        let corners = [outline.left_top(), outline.right_top(), outline.right_bottom(), outline.left_bottom()];
        (0..4)
            .map(|i| {
                let (a, b) = (corners[i], corners[(i + 1) % 4]);
                let side = mix(seed ^ (i as u64 + 1));
                let length = (b - a).length();
                if length < 0.5 {
                    return Shape::Noop;
                }
                let along = (b - a) / length;
                // each end a little past the corner, as a pen runs on
                let past = |k: u64| (OVERSHOOT * amount * (0.6 + 0.4 * unit(side, k as i64))).min(0.15 * length);
                let ends = [a - along * past(7), b + along * past(9)];
                line(wobble(&ends, side, amount), stroke.width, color)
            })
            .collect()
    };
    let mut lines = pass(seed, stroke.color);
    lines.extend(pass(mix(seed ^ 0xD0), lighter(stroke.color)));
    with_fills(fill, lines)
}

/// A circle: its fill as it was, its outline round once and a little more,
/// not quite round.
fn circle(c: CircleShape, amount: f32) -> Shape {
    let seed = mix(hash_f(c.radius * 4.0) ^ u64::from(color_bits(c.stroke.color)) ^ 0x51);
    let n = (c.radius * 1.2).clamp(12.0, 64.0) as usize;
    let lean = 0.035 * amount * unit(seed, 3);
    let points: Vec<Pos2> = (0..n)
        .map(|k| {
            let a = TAU * k as f32 / n as f32;
            c.center + vec2(a.cos() * c.radius * (1.0 + lean), a.sin() * c.radius * (1.0 - lean))
        })
        .collect();
    let mut fill = c;
    let stroke = fill.stroke;
    fill.stroke = Stroke::NONE;
    let fill = (fill.fill != Color32::TRANSPARENT).then_some(Shape::Circle(fill));
    let again = line(closing(&points, mix(seed ^ 0xD0), amount), stroke.width, lighter(stroke.color));
    with_fills(fill, vec![line(closing(&points, seed, amount), stroke.width, stroke.color), again])
}

/// A colour gone over the second time: `AGAIN` as opaque.
fn lighter(c: Color32) -> Color32 {
    c.gamma_multiply(AGAIN)
}

/// A path: its fill (if closed) as it was, its outline hand drawn.
fn path(p: PathShape, amount: f32) -> Shape {
    let Some(color) = solid(&p.stroke) else { return Shape::Path(p) };
    if p.stroke.width <= 0.0 || p.points.len() < 2 {
        return Shape::Path(p);
    }
    let seed = form(&p.points, color, p.stroke.width);
    let points = if p.closed { closing(&p.points, seed, amount) } else { wobble(&p.points, seed, amount) };
    let fill = filled(p.fill, p.points, p.closed);
    with_fill(fill, line(points, p.stroke.width, color))
}

fn filled(fill: Color32, points: Vec<Pos2>, closed: bool) -> Option<Shape> {
    (closed && fill != Color32::TRANSPARENT).then(|| Shape::Path(PathShape::convex_polygon(points, fill, Stroke::NONE)))
}

fn line(points: Vec<Pos2>, width: f32, color: Color32) -> Shape {
    Shape::Path(PathShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: PathStroke { width, color: ColorMode::Solid(color), kind: StrokeKind::Middle },
    })
}

fn with_fill(fill: Option<Shape>, line: Shape) -> Shape {
    match fill {
        Some(fill) => Shape::Vec(vec![fill, line]),
        None => line,
    }
}

fn with_fills(fill: Option<Shape>, lines: Vec<Shape>) -> Shape {
    Shape::Vec(fill.into_iter().chain(lines).collect())
}

/// A box with rounded corners as a closed line, from its top left going
/// right.
fn rounded(r: Rect, radius: f32) -> Vec<Pos2> {
    let radius = radius.min(r.width() / 2.0).min(r.height() / 2.0);
    let mut points = Vec::with_capacity(24);
    let corner = |points: &mut Vec<Pos2>, centre: Pos2, from: f32| {
        for k in 0..=4 {
            let a = from + TAU / 4.0 * k as f32 / 4.0;
            points.push(centre + radius * vec2(a.cos(), a.sin()));
        }
    };
    corner(&mut points, Pos2::new(r.right() - radius, r.top() + radius), -TAU / 4.0);
    corner(&mut points, Pos2::new(r.right() - radius, r.bottom() - radius), 0.0);
    corner(&mut points, Pos2::new(r.left() + radius, r.bottom() - radius), TAU / 4.0);
    corner(&mut points, Pos2::new(r.left() + radius, r.top() + radius), TAU / 2.0);
    points
}

/// A closed outline drawn as a hand closes one: from a point of its own
/// round once and on a little past it.
fn closing(points: &[Pos2], seed: u64, amount: f32) -> Vec<Pos2> {
    let n = points.len();
    if n < 3 {
        return wobble(points, seed, amount);
    }
    let start = (mix(seed ^ 0x77) % n as u64) as usize;
    let mut round: Vec<Pos2> = (0..=n).map(|k| points[(start + k) % n]).collect();
    // on past the start, as far as the overlap goes
    let mut gone = 0.0;
    let overlap = OVERLAP * amount * (0.5 + 0.5 * unit(seed, 5).abs());
    for k in 1..n {
        let (a, b) = (points[(start + k - 1) % n], points[(start + k) % n]);
        let piece = (b - a).length();
        if gone + piece >= overlap {
            let t = if piece > 0.0 { (overlap - gone) / piece } else { 0.0 };
            round.push(a + (b - a) * t);
            break;
        }
        gone += piece;
        round.push(b);
    }
    wobble(&round, seed, amount)
}

/// `points` (an open line) as a hand draws it: every piece cut to `STEP`
/// at most, each point moved off the line by a smooth wobble of its place
/// along it.
fn wobble(points: &[Pos2], seed: u64, amount: f32) -> Vec<Pos2> {
    let total: f32 = points.windows(2).map(|w| (w[1] - w[0]).length()).sum();
    if points.len() < 2 || total < 1e-3 {
        return points.to_vec();
    }
    // short lines stray less: a jack's ring, an icon
    let stray = STRAY * amount * (total / WAVE).min(1.0);
    let mut out = Vec::with_capacity(points.len() + (total / STEP) as usize + 2);
    let mut along = 0.0;
    for (i, w) in points.windows(2).enumerate() {
        let (a, b) = (w[0], w[1]);
        let piece = (b - a).length();
        let normal = normal_of(b - a);
        // at a corner the normals of the pieces meeting there, between
        let at_start = match i {
            0 => normal,
            _ => (normal + normal_of(a - points[i - 1])).normalized(),
        };
        out.push(a + at_start * stray * noise(seed, along));
        let cuts = (piece / STEP).ceil() as usize;
        for k in 1..cuts {
            let t = k as f32 / cuts as f32;
            out.push(a + (b - a) * t + normal * stray * noise(seed, along + piece * t));
        }
        along += piece;
    }
    let last = points[points.len() - 1];
    let normal = normal_of(last - points[points.len() - 2]);
    out.push(last + normal * stray * noise(seed, along));
    out
}

fn normal_of(d: Vec2) -> Vec2 {
    let n = vec2(-d.y, d.x);
    let l = n.length();
    if l > 0.0 {
        n / l
    } else {
        Vec2::ZERO
    }
}

/// A smooth wobble, −1.35…1.35, at `s` along a line: two waves of value
/// noise, the second a third as long and half as big.
fn noise(seed: u64, s: f32) -> f32 {
    let wave = |seed: u64, x: f32| {
        let (i, f) = (x.floor(), x - x.floor());
        let f = f * f * (3.0 - 2.0 * f);
        let (a, b) = (unit(seed, i as i64), unit(seed, i as i64 + 1));
        a + (b - a) * f
    };
    wave(seed, s / WAVE) + 0.35 * wave(mix(seed ^ 0xA5), 3.0 * s / WAVE)
}

/// A number −1…1 from `seed` and `i`.
fn unit(seed: u64, i: i64) -> f32 {
    let h = mix(seed ^ (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    (h >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}

/// SplitMix64's mixer.
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fn hash_f(x: f32) -> u64 {
    mix(u64::from((x * 2.0).round() as i32 as u32))
}

fn color_bits(c: Color32) -> u32 {
    u32::from_le_bytes(c.to_array())
}

/// A seed from a line's form: its points relative to its first (to half a
/// pixel), its colour and width; where it is doesn't count.
fn form(points: &[Pos2], color: Color32, width: f32) -> u64 {
    let first = points[0];
    let mut h = mix(u64::from(color_bits(color)) ^ hash_f(width).rotate_left(7));
    for p in points.iter().skip(1).step_by((points.len() / 8).max(1)) {
        h = mix(h ^ hash_f(p.x - first.x) ^ hash_f(p.y - first.y).rotate_left(32));
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::epaint::CubicBezierShape;
    use egui::pos2;

    fn points_of(shape: &Shape) -> Vec<Pos2> {
        match shape {
            Shape::Path(p) => p.points.clone(),
            Shape::Vec(v) => v.iter().flat_map(points_of).collect(),
            _ => Vec::new(),
        }
    }

    /// Lines wobble, but only a little; a shape moved keeps its wobble (so
    /// scrolling doesn't stir the lines); nothing changes at 0.
    #[test]
    fn lines_wobble_a_little_and_the_same_wherever_they_are() {
        let box_at = |at: Pos2| {
            Shape::Rect(RectShape::new(
                Rect::from_min_size(at, vec2(240.0, 90.0)),
                0,
                Color32::WHITE,
                Stroke::new(1.0, Color32::BLACK),
                StrokeKind::Inside,
            ))
        };
        let drawn = |shape: Shape, amount: f32| {
            let mut shapes = [ClippedShape { clip_rect: Rect::EVERYTHING, shape }];
            roughen(&mut shapes, amount);
            let [ClippedShape { shape, .. }] = shapes;
            shape
        };
        let here = points_of(&drawn(box_at(pos2(10.0, 10.0)), 1.0));
        let there = points_of(&drawn(box_at(pos2(310.5, 47.25)), 1.0));
        assert!(here.len() > 30, "cut up to wobble: {}", here.len());
        let moved = vec2(300.5, 37.25);
        assert!(here.iter().zip(&there).all(|(a, b)| ((*a + moved) - *b).length() < 1e-3), "moved, the same lines");
        let outline = Rect::from_min_size(pos2(10.0, 10.0), vec2(240.0, 90.0)).shrink(0.5);
        let off = |p: &Pos2| {
            let inside = outline.distance_to_pos(*p);
            let to_edge = (p.x - outline.left()).abs().min((p.x - outline.right()).abs()).min((p.y - outline.top()).abs()).min((p.y - outline.bottom()).abs());
            inside.max(to_edge)
        };
        let most = here.iter().map(off).fold(0.0, f32::max);
        assert!(most > 0.2 && most < 2.5 * STRAY + OVERSHOOT, "off the box by {most} px at most");
        assert!(matches!(drawn(box_at(pos2(0.0, 0.0)), 0.0), Shape::Rect(_)), "at 0: as it was");

        let cable = Shape::CubicBezier(CubicBezierShape::from_points_stroke(
            [pos2(0.0, 0.0), pos2(40.0, 120.0), pos2(160.0, 120.0), pos2(200.0, 0.0)],
            false,
            Color32::TRANSPARENT,
            Stroke::new(2.0, Color32::RED),
        ));
        assert!(points_of(&drawn(cable, 1.0)).len() > 10, "a cable, drawn as a line by hand");
        let text_like = Shape::Noop;
        assert!(matches!(drawn(text_like, 1.0), Shape::Noop));
    }
}
