//! Draws egui frames into an image, with no window and no GPU: for pictures
//! of an editor (documentation, the store) and for checking a layout in
//! tests, including where no screen can be read.
//!
//! A small triangle rasteriser that paints the way egui's OpenGL painter
//! does (the one under nih-plug's editors): texture times vertex colour, in
//! gamma space, blended with premultiplied alpha. Textures are sampled
//! bilinearly, clamped to their edges or repeating as their options say;
//! mipmaps and nearest sampling aren't drawn.

use std::collections::HashMap;

use crate::egui::epaint::textures::TextureWrapMode;
use crate::egui::epaint::{ClippedShape, ImageData, ImageDelta, Primitive, TextureId};
use crate::egui::{Color32, Context, Event, FullOutput, Pos2, RawInput, Rect, Vec2, ViewportId};

/// An RGB picture, row by row from the top.
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub rgb: Vec<u8>,
}

impl Image {
    pub fn pixel(&self, x: usize, y: usize) -> Color32 {
        let i = 3 * (y * self.width + x);
        Color32::from_rgb(self.rgb[i], self.rgb[i + 1], self.rgb[i + 2])
    }

    /// The picture as a PNG file (stored, not compressed: this is for
    /// looking at, not for shipping).
    pub fn to_png(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity((3 * self.width + 1) * self.height);
        for row in self.rgb.chunks(3 * self.width) {
            raw.push(0); // no filter
            raw.extend_from_slice(row);
        }
        let mut zlib = vec![0x78, 0x01];
        let blocks = raw.chunks(0xFFFF).count();
        for (i, block) in raw.chunks(0xFFFF).enumerate() {
            let len = block.len() as u16;
            zlib.push(u8::from(i + 1 == blocks));
            zlib.extend_from_slice(&len.to_le_bytes());
            zlib.extend_from_slice(&(!len).to_le_bytes());
            zlib.extend_from_slice(block);
        }
        zlib.extend_from_slice(&adler32(&raw).to_be_bytes());

        let mut header = Vec::with_capacity(13);
        header.extend_from_slice(&(self.width as u32).to_be_bytes());
        header.extend_from_slice(&(self.height as u32).to_be_bytes());
        header.extend_from_slice(&[8, 2, 0, 0, 0]); // 8-bit RGB
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        for (kind, data) in [(b"IHDR", &header), (b"IDAT", &zlib), (b"IEND", &Vec::new())] {
            png.extend_from_slice(&(data.len() as u32).to_be_bytes());
            let start = png.len();
            png.extend_from_slice(kind);
            png.extend_from_slice(data);
            let crc = crc32(&png[start..]);
            png.extend_from_slice(&crc.to_be_bytes());
        }
        png
    }
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in data.chunks(5552) {
        for &x in chunk {
            a += u32::from(x);
            b += a;
        }
        a %= 65521;
        b %= 65521;
    }
    (b << 16) | a
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &x in data {
        crc ^= u32::from(x);
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

/// A texture as egui's painter holds it: premultiplied, gamma-space RGBA.
struct Texture {
    size: [usize; 2],
    pixels: Vec<[f32; 4]>,
    wrap: TextureWrapMode,
}

impl Texture {
    fn from_delta(delta: &ImageDelta) -> Self {
        let image = &delta.image;
        let colors: Vec<Color32> = match image {
            ImageData::Color(c) => c.pixels.clone(),
        };
        let pixels = colors.iter().map(|c| c.to_array().map(|v| f32::from(v) / 255.0)).collect();
        Texture { size: image.size(), pixels, wrap: delta.options.wrap_mode }
    }

    fn apply(&mut self, delta: &ImageDelta) {
        let patch = Texture::from_delta(delta);
        let Some([x0, y0]) = delta.pos else {
            *self = patch;
            return;
        };
        for y in 0..patch.size[1] {
            for x in 0..patch.size[0] {
                self.pixels[(y0 + y) * self.size[0] + x0 + x] = patch.pixels[y * patch.size[0] + x];
            }
        }
    }

    /// Bilinear, clamped to the edges or repeating, as the texture's
    /// options say.
    fn sample(&self, uv: Pos2) -> [f32; 4] {
        let [w, h] = self.size;
        let fx = uv.x * w as f32 - 0.5;
        let fy = uv.y * h as f32 - 0.5;
        let (x0, y0) = (fx.floor(), fy.floor());
        let (tx, ty) = (fx - x0, fy - y0);
        let wrap = self.wrap;
        let index = |v: f32, n: usize| -> usize {
            let n = n as i64;
            let v = v as i64;
            let i = match wrap {
                TextureWrapMode::ClampToEdge => v.clamp(0, n - 1),
                TextureWrapMode::Repeat => v.rem_euclid(n),
                TextureWrapMode::MirroredRepeat => {
                    let m = v.rem_euclid(2 * n);
                    if m < n { m } else { 2 * n - 1 - m }
                }
            };
            i as usize
        };
        let at = |x: f32, y: f32| self.pixels[index(y, h) * w + index(x, w)];
        let (a, b, c, d) = (at(x0, y0), at(x0 + 1.0, y0), at(x0, y0 + 1.0), at(x0 + 1.0, y0 + 1.0));
        std::array::from_fn(|i| {
            let top = a[i] + (b[i] - a[i]) * tx;
            let bottom = c[i] + (d[i] - c[i]) * tx;
            top + (bottom - top) * ty
        })
    }
}

/// An egui context drawing into pictures of a fixed size.
pub struct Snapshot {
    pub ctx: Context,
    size: Vec2,
    pixels_per_point: f32,
    textures: HashMap<TextureId, Texture>,
    time: f64,
    /// The last frame asked for another right away (an animation runs).
    animating: bool,
    /// The cursor the last frame asked for.
    pub cursor: egui::CursorIcon,
    /// Files dragged over the window from the system, while they are, and
    /// files let go on it (the next frame takes them).
    pub hovered: Vec<std::path::PathBuf>,
    pub dropped: Vec<std::path::PathBuf>,
    /// The modifier keys held (Ctrl for a shortcut), each frame.
    pub modifiers: egui::Modifiers,
    /// The screen's safe area, if it has one (a phone's: its camera's strip,
    /// its home indicator's).
    pub safe_area: Option<egui::SafeAreaInsets>,
}

impl Snapshot {
    /// A fresh context for a `size` window (in points).
    pub fn new(size: Vec2, pixels_per_point: f32) -> Self {
        Snapshot {
            ctx: Context::default(),
            size,
            pixels_per_point,
            textures: HashMap::new(),
            time: 0.0,
            animating: false,
            cursor: egui::CursorIcon::Default,
            hovered: Vec::new(),
            dropped: Vec::new(),
            modifiers: egui::Modifiers::NONE,
            safe_area: None,
        }
    }

    /// Runs frames with no input until the picture settles: overlays fade
    /// in, hovers light up, text sizes itself. Gives up after a second.
    pub fn settle(&mut self, mut ui: impl FnMut(&mut egui::Ui)) -> Image {
        let mut image = self.run(Vec::new(), &mut ui);
        for _ in 0..60 {
            if !self.animating {
                break;
            }
            image = self.run(Vec::new(), &mut ui);
        }
        image
    }

    /// The frames `settle` runs, undrawn: for following an interface to
    /// where it settles without the cost of its pictures (drawing them in
    /// software, unoptimised, takes CI's tests minutes a picture).
    pub fn settle_undrawn(&mut self, mut ui: impl FnMut(&mut egui::Ui)) {
        self.step(Vec::new(), &mut ui);
        for _ in 0..60 {
            if !self.animating {
                break;
            }
            self.step(Vec::new(), &mut ui);
        }
    }

    /// Runs one frame with these input events without drawing it: for
    /// following what an interface does, not what it looks like (drawing is
    /// most of a frame's cost here).
    pub fn step(&mut self, events: Vec<Event>, ui: impl FnMut(&mut egui::Ui)) {
        let output = self.frame(events, ui);
        for id in &output.textures_delta.free {
            self.textures.remove(id);
        }
    }

    /// Runs one frame with these input events and gives what it would draw,
    /// undrawn: each shape with the rectangle it's cut to, in the order
    /// they're painted. For asking where something is drawn and how much of
    /// it shows, without the cost of a picture.
    pub fn shapes(&mut self, events: Vec<Event>, ui: impl FnMut(&mut egui::Ui)) -> Vec<ClippedShape> {
        let output = self.frame(events, ui);
        for id in &output.textures_delta.free {
            self.textures.remove(id);
        }
        output.shapes
    }

    /// One frame with these input events: what egui gives back, the
    /// textures it set kept for drawing.
    fn frame(&mut self, events: Vec<Event>, ui: impl FnMut(&mut egui::Ui)) -> FullOutput {
        let mut input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
            time: Some(self.time),
            events,
            hovered_files: self.hovered.iter().map(|p| egui::HoveredFile { path: Some(p.clone()), ..Default::default() }).collect(),
            dropped_files: std::mem::take(&mut self.dropped)
                .into_iter()
                .map(|p| egui::DroppedFile { path: Some(p), ..Default::default() })
                .collect(),
            modifiers: self.modifiers,
            focused: true,
            safe_area_insets: self.safe_area,
            ..Default::default()
        };
        input.viewports.entry(ViewportId::ROOT).or_default().native_pixels_per_point = Some(self.pixels_per_point);
        self.time += 1.0 / 60.0;

        let output = self.ctx.run_ui(input, ui);
        self.animating = output.viewport_output.get(&ViewportId::ROOT).is_some_and(|v| v.repaint_delay.is_zero());
        self.cursor = output.platform_output.cursor_icon;
        for (id, delta) in &output.textures_delta.set {
            match self.textures.get_mut(id) {
                Some(texture) if delta.pos.is_some() => texture.apply(delta),
                _ => {
                    self.textures.insert(*id, Texture::from_delta(delta));
                }
            }
        }
        output
    }

    /// Runs one frame with these input events and draws it. Most pictures
    /// want `settle` instead.
    pub fn run(&mut self, events: Vec<Event>, ui: impl FnMut(&mut egui::Ui)) -> Image {
        let output = self.frame(events, ui);
        let ppp = output.pixels_per_point;
        // hand drawn, in a theme that is (as the window draws it)
        let mut shapes = output.shapes;
        crate::sketch::roughen(&mut shapes, crate::theme::sketch());
        let primitives = self.ctx.tessellate(shapes, ppp);

        let width = (self.size.x * ppp).round() as usize;
        let height = (self.size.y * ppp).round() as usize;
        let mut target = Target { width, pixels: vec![[0.0; 4]; width * height] };
        for clipped in &primitives {
            let Primitive::Mesh(mesh) = &clipped.primitive else { continue };
            let Some(texture) = self.textures.get(&mesh.texture_id) else { continue };
            let clip = clipped.clip_rect;
            let clip = [
                (clip.min.x * ppp).round().clamp(0.0, width as f32) as usize,
                (clip.min.y * ppp).round().clamp(0.0, height as f32) as usize,
                (clip.max.x * ppp).round().clamp(0.0, width as f32) as usize,
                (clip.max.y * ppp).round().clamp(0.0, height as f32) as usize,
            ];
            for tri in mesh.indices.chunks_exact(3) {
                let v = [0, 1, 2].map(|k| &mesh.vertices[tri[k] as usize]);
                target.triangle(v, ppp, clip, texture);
            }
        }
        for id in &output.textures_delta.free {
            self.textures.remove(id);
        }

        let rgb = target
            .pixels
            .iter()
            .flat_map(|p| [p[0], p[1], p[2]].map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8))
            .collect();
        Image { width, height, rgb }
    }
}

struct Target {
    width: usize,
    pixels: Vec<[f32; 4]>,
}

/// Twice the signed area of (a, b, c).
fn edge(a: Pos2, b: Pos2, c: Pos2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// A pixel centre exactly on an edge belongs to one of the two triangles
/// sharing it, never both: to the one that runs the edge this way.
fn owns(a: Pos2, b: Pos2) -> bool {
    let d = b - a;
    d.y > 0.0 || (d.y == 0.0 && d.x > 0.0)
}

impl Target {
    fn triangle(&mut self, v: [&crate::egui::epaint::Vertex; 3], ppp: f32, clip: [usize; 4], texture: &Texture) {
        let mut p = v.map(|v| Pos2::new(v.pos.x * ppp, v.pos.y * ppp));
        let mut v = v;
        let mut area = edge(p[0], p[1], p[2]);
        if area == 0.0 {
            return;
        }
        if area < 0.0 {
            p.swap(1, 2);
            v.swap(1, 2);
            area = -area;
        }
        let min_x = (p.iter().map(|q| q.x).fold(f32::MAX, f32::min).floor().max(0.0) as usize).max(clip[0]);
        let min_y = (p.iter().map(|q| q.y).fold(f32::MAX, f32::min).floor().max(0.0) as usize).max(clip[1]);
        let max_x = (p.iter().map(|q| q.x).fold(f32::MIN, f32::max).ceil().max(0.0) as usize).min(clip[2]);
        let max_y = (p.iter().map(|q| q.y).fold(f32::MIN, f32::max).ceil().max(0.0) as usize).min(clip[3]);
        let colors = v.map(|v| v.color.to_array().map(|c| f32::from(c) / 255.0));
        let edges = [(p[1], p[2]), (p[2], p[0]), (p[0], p[1])];
        for y in min_y..max_y {
            for x in min_x..max_x {
                let c = Pos2::new(x as f32 + 0.5, y as f32 + 0.5);
                let w = edges.map(|(a, b)| edge(a, b, c));
                if !(0..3).all(|i| w[i] > 0.0 || (w[i] == 0.0 && owns(edges[i].0, edges[i].1))) {
                    continue;
                }
                let l = w.map(|w| w / area);
                let uv = Pos2::new(
                    l[0] * v[0].uv.x + l[1] * v[1].uv.x + l[2] * v[2].uv.x,
                    l[0] * v[0].uv.y + l[1] * v[1].uv.y + l[2] * v[2].uv.y,
                );
                let t = texture.sample(uv);
                let src: [f32; 4] =
                    std::array::from_fn(|i| t[i] * (l[0] * colors[0][i] + l[1] * colors[1][i] + l[2] * colors[2][i]));
                let dst = &mut self.pixels[y * self.width + x];
                for i in 0..4 {
                    dst[i] = src[i] + dst[i] * (1.0 - src[3]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::{self, CornerRadius};

    #[test]
    fn paints_a_rect_and_text() {
        let mut snap = Snapshot::new(Vec2::new(64.0, 32.0), 1.0);
        let draw = |ui: &mut egui::Ui| {
            egui::CentralPanel::default().frame(egui::Frame::new().fill(Color32::from_rgb(0x17, 0x17, 0x17))).show_inside(
                ui,
                |ui| {
                    let r = Rect::from_min_size(Pos2::new(8.0, 8.0), Vec2::new(16.0, 16.0));
                    ui.painter().rect_filled(r, CornerRadius::ZERO, Color32::from_rgb(0, 0xff, 0x41));
                    ui.painter().text(
                        Pos2::new(40.0, 16.0),
                        egui::Align2::CENTER_CENTER,
                        "W",
                        egui::FontId::proportional(16.0),
                        Color32::WHITE,
                    );
                },
            );
        };
        let image = snap.settle(draw);
        assert_eq!((image.width, image.height), (64, 32));
        assert_eq!(image.pixel(2, 2), Color32::from_rgb(0x17, 0x17, 0x17));
        assert_eq!(image.pixel(16, 16), Color32::from_rgb(0, 0xff, 0x41));
        // the glyph put some light near its centre
        let lit = (32..48).flat_map(|x| (8..24).map(move |y| (x, y))).filter(|&(x, y)| image.pixel(x, y).r() > 0x80);
        assert!(lit.count() > 5);
        let png = image.to_png();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn checksums() {
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }
}
