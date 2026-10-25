//! The palette and type (PLUGIN_GUIDE.md "Colour palette", "Typography"),
//! and `install`, which sets them on an egui context; and themes, which
//! change the palette while the plugin runs.
//!
//! Every colour has a role (`Role`), read with the role's function:
//! `ACCENT()`, `PANEL()` … (named as the constants they once were, so
//! drawing code reads the same). Built in: Matrix (the family's own look,
//! phosphor green on black; called "Phosphor", then "The Matrix", before:
//! names still understood), Dusk (soft and dark) and Paper (light). A theme file is JSON: a name, a
//! base theme and the colours it changes (`Theme::from_json`), and maybe a
//! picture behind the patch (`Background`, read with `background()`).
//!
//! The theme chosen (`apply`) is the process's: every window wears it.
//! Each drawing thread keeps its own copy of the colours, and takes the
//! chosen theme at the start of a frame (`sync`), so reading a colour is
//! a thread-local load and never waits; a thread that doesn't sync (a
//! test drawing a picture) keeps what it has (`apply_here`).

use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use egui::{self, pos2, Color32, FontData, FontDefinitions, FontFamily, FontId, Rect, Stroke, Vec2};
use serde::Deserialize;

/// The family's own theme's name, and the ones it had before (still
/// understood in settings and as a theme file's base).
pub const MATRIX_NAME: &str = "Matrix";

/// The theme the windows wear when none was chosen (2026-10-15, the user:
/// "Dusk should be the default theme"; Matrix until then).
pub const DEFAULT_NAME: &str = "Dusk";
pub const OLD_MATRIX_NAMES: [&str; 2] = ["Phosphor", "The Matrix"];

/// Declares the roles: each one's function, its key in theme files, and
/// its colour in the built-in themes (0xRRGGBBAA): Matrix, Dusk, Paper.
macro_rules! roles {
    ($($(#[$doc:meta])* $name:ident $key:literal $matrix:literal $dusk:literal $paper:literal,)*) => {
        /// A colour's part in the look.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum Role {
            $($name,)*
        }

        /// How many roles there are.
        pub const COUNT: usize = [$($key,)*].len();

        impl Role {
            pub const ALL: [Role; COUNT] = [$(Role::$name,)*];

            /// Its name in theme files.
            pub fn key(self) -> &'static str {
                match self {
                    $(Role::$name => $key,)*
                }
            }

            pub fn from_key(key: &str) -> Option<Role> {
                match key {
                    $($key => Some(Role::$name),)*
                    _ => None,
                }
            }
        }

        const MATRIX: [u32; COUNT] = [$($matrix,)*];
        const DUSK: [u32; COUNT] = [$($dusk,)*];
        const PAPER: [u32; COUNT] = [$($paper,)*];

        $(
            $(#[$doc])*
            #[allow(non_snake_case)]
            #[inline]
            pub fn $name() -> Color32 {
                color(Role::$name)
            }
        )*
    };
}

roles! {
    /// Window background.
    WINDOW "window" 0x171717ff 0x1c2025ff 0xefebe3ff,
    /// Sections and overlay cards, and their border.
    PANEL "panel" 0x212121ff 0x242930ff 0xf8f5efff,
    PANEL_BORDER "panel_border" 0x2c2c2cff 0x30363fff 0xd8d1c5ff,
    /// Recessed wells: the preset bar, step pickers, the patch canvas.
    WELL "well" 0x101010ff 0x171a1eff 0xe6e1d7ff,
    /// A module's border on the patch canvas. (On Paper a module is nearly
    /// the canvas's own colour, so its border is what sets it off.)
    MODULE_BORDER "module_border" 0x2a2a2aff 0x30363fff 0xc0b7a8ff,
    /// Lists and the browser's bank sidebar, and their border.
    LIST "list" 0x161616ff 0x1e2227ff 0xf4f0e9ff,
    LIST_BORDER "list_border" 0x2a2a2aff 0x30363fff 0xd5cec2ff,
    /// Buttons: in panels, in the preset bar; their border; hovered.
    BUTTON "button" 0x1a1a1aff 0x20252bff 0xf1ede6ff,
    BUTTON_BAR "button_bar" 0x212121ff 0x262c33ff 0xf8f5efff,
    BUTTON_BORDER "button_border" 0x303030ff 0x363d47ff 0xcbc3b5ff,
    BUTTON_HOVER "button_hover" 0x2c2c2cff 0x2c333bff 0xe8e2d8ff,
    BUTTON_HOVER_BORDER "button_hover_border" 0x444444ff 0x48515cff 0xb3aa9bff,
    /// Hovered rows, banks and toggle pills.
    ROW_HOVER "row_hover" 0x262626ff 0x2a3038ff 0xebe6ddff,
    /// The current preset row, the selected bank.
    SELECTED "selected" 0x0e2413ff 0x23393aff 0xd5e7dfff,
    /// The brand's hover background.
    BRAND_HOVER "brand_hover" 0x1f1f1fff 0x242930ff 0xe9e4dbff,
    /// Vertical rules between groups.
    DIVIDER "divider" 0x2e2e2eff 0x343b44ff 0xd3cbbeff,
    /// The brand, the patch's name, section marks, knob arcs, lit LEDs.
    ACCENT "accent" 0x00ff41ff 0x88c8aeff 0x26705fff,
    /// Primary button labels, folder paths, checked toggle labels.
    ACCENT_SOFT "accent_soft" 0xb8ffc8ff 0xbfe3d4ff 0x21604fff,
    /// Knob values, info values.
    VALUE "value" 0xd6f5dcff 0xd5e6dfff 0x1e3b34ff,
    /// Text.
    TEXT "text" 0xc8c8c8ff 0xc8cdd4ff 0x2b2824ff,
    TEXT_BUTTON "text_button" 0xb8b8b8ff 0xbac0c9ff 0x37332dff,
    /// The brand's second word.
    BRAND_SECOND "brand_second" 0xd0d0d0ff 0xc8cdd4ff 0x3b3731ff,
    /// Section titles, headings, toggle labels.
    SECONDARY "secondary" 0x9a9a9aff 0x9ca5b0ff 0x5c574fff,
    /// Knob labels.
    LABEL "label" 0x858585ff 0x8a949fff 0x69635aff,
    /// Captions, taglines, info keys.
    MUTED "muted" 0x6e6e6eff 0x737d88ff 0x80796eff,
    /// Subtitles, counts, unset values.
    FAINT "faint" 0x5e5e5eff 0x5f6974ff 0x9a9387ff,
    /// The knob ring's off part, an unlit LED.
    RING "ring" 0x3a3a3aff 0x3d454fff 0xcdc5b8ff,
    /// Inside the knob ring.
    KNOB_WELL "knob_well" 0x1b1b1bff 0x20252bff 0xf1ede6ff,
    WARNING "warning" 0xffb347ff 0xe2b56eff 0xa8620dff,
    SUCCESS "success" 0x6fdc8cff 0x8fd0a2ff 0x2e7d47ff,
    ERROR "error" 0xff7a6bff 0xe3897eff 0xb03e35ff,
    /// The patch's cables in the G2's seven colours (a cable keeps the
    /// colour's number), and jacks by their signal: audio red, control
    /// blue, logic yellow.
    CABLE_RED "cable_red" 0xd8554fff 0xd97a72ff 0xbe4239ff,
    CABLE_BLUE "cable_blue" 0x4f8fd8ff 0x72a0d9ff 0x3565b0ff,
    CABLE_YELLOW "cable_yellow" 0xd8c24fff 0xd4c170ff 0xa0800cff,
    CABLE_ORANGE "cable_orange" 0xe08a3cff 0xdd9d66ff 0xbd6519ff,
    CABLE_GREEN "cable_green" 0x5cb85cff 0x82c282ff 0x3b8a3bff,
    CABLE_PURPLE "cable_purple" 0xa66ad8ff 0xab93d9ff 0x7b4db3ff,
    CABLE_WHITE "cable_white" 0xd0d0d0ff 0xcdd3daff 0x55524cff,
    /// The legacy (G2) modules' mark in the module lists, and a patch's
    /// legacy format in the patch browser.
    LEGACY "legacy" 0x9c8860ff 0xb09c78ff 0x8a6a36ff,
    /// Under cards and floating panels.
    SHADOW "shadow" 0x00000099 0x00000099 0x00000033,
    /// Over the window behind a modal card.
    DIM "dim" 0x000000b8 0x0b0d10b8 0x2b241a52,
}

/// A theme: its name, a colour for every role, and maybe a picture behind
/// the patch; how hand drawn its outlines are (`sketch`, 0: not at all,
/// `sketch::roughen`), and a font of its own (a file, or `HAND_FONT` by
/// its name) drawn `font_scale` times as big.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub name: String,
    colors: [u32; COUNT],
    pub background: Option<Background>,
    pub sketch: f32,
    pub font: Option<PathBuf>,
    pub font_scale: f32,
}

/// The hand-written font built in (`assets/coming-soon`: the Apache
/// License 2.0, its file beside it), the Sketch theme's; a theme file names
/// it by this name instead of a file.
pub const HAND_FONT: &str = "Coming Soon";
const HAND_FONT_DATA: &[u8] = include_bytes!("../assets/coming-soon/ComingSoon-Regular.ttf");

/// Whether a theme's `font` is the one built in (by its name), not a file.
pub fn is_built_in_font(font: &Path) -> bool {
    font.to_str().is_some_and(|f| f.trim().eq_ignore_ascii_case(HAND_FONT))
}

/// The theme files' shape.
#[derive(Deserialize)]
struct File {
    #[serde(default)]
    name: String,
    #[serde(default)]
    base: Option<String>,
    #[serde(default)]
    colors: BTreeMap<String, String>,
    #[serde(default)]
    background: Option<serde_json::Value>,
    #[serde(default)]
    sketch: Option<f32>,
    #[serde(default)]
    font: Option<String>,
    #[serde(default)]
    font_scale: Option<f32>,
}

/// A picture behind the patch canvas (a theme file's `background`): a PNG
/// or JPEG file, how big it's drawn, whether it repeats, where it sits, whether it
/// stays put as the patch scrolls, how much it shows, and whether it's
/// smoothed when scaled.
#[derive(Clone, Debug, PartialEq)]
pub struct Background {
    /// The file, as the theme names it (relative to the theme's folder
    /// until `resolve_in` makes it whole).
    pub image: PathBuf,
    pub size: Size,
    /// It repeats across, and down.
    pub repeat: [bool; 2],
    /// Where it sits in its room, across and down: 0 left (top), ½ the
    /// middle, 1 right (bottom).
    pub position: [f32; 2],
    /// It stays where it is while the patch scrolls and zooms, sized by the
    /// canvas (the window); else it moves and zooms with the patch.
    pub fixed: bool,
    /// How much it shows over the canvas's colour (`well`), 0 to 1.
    pub opacity: f32,
    /// Smoothed when it's drawn bigger than it is; else its pixels stay
    /// hard (pixel art).
    pub smooth: bool,
}

/// How big a background is drawn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Size {
    /// It covers its room, its sides in proportion (what's over is cut).
    Cover,
    /// It fits in its room, its sides in proportion.
    Contain,
    /// It's its room's size.
    Stretch,
    /// Its own pixels times this.
    Scale(f32),
}

impl Background {
    const KEYS: [&'static str; 7] = ["image", "size", "repeat", "position", "fixed", "opacity", "smooth"];

    /// A theme file's `background`, what was wrong in it put in `problems`
    /// (a value that isn't one keeps the default); none without an image.
    fn from_value(value: &serde_json::Value, problems: &mut Vec<String>) -> Option<Background> {
        use serde_json::Value;
        let mut said = |text: String| problems.push(format!("background: {text}"));
        let Value::Object(fields) = value else {
            said("isn't a list of settings ({\"image\": \"…\"})".into());
            return None;
        };
        for key in fields.keys().filter(|k| !Self::KEYS.contains(&k.as_str())) {
            said(format!("\"{key}\" is no setting of a background"));
        }
        let image = match fields.get("image") {
            Some(Value::String(s)) if !s.trim().is_empty() => PathBuf::from(s.trim()),
            _ => {
                said("needs an \"image\": a picture's file name (PNG or JPEG)".into());
                return None;
            }
        };
        let mut background = Background {
            image,
            size: Size::Cover,
            repeat: [false; 2],
            position: [0.5; 2],
            fixed: true,
            opacity: 1.0,
            smooth: true,
        };
        match fields.get("size") {
            None => {}
            Some(Value::String(s)) => match s.trim().to_ascii_lowercase().as_str() {
                "cover" => background.size = Size::Cover,
                "contain" => background.size = Size::Contain,
                "stretch" => background.size = Size::Stretch,
                "original" => background.size = Size::Scale(1.0),
                _ => said(format!("size \"{s}\": cover, contain, stretch, original or a number")),
            },
            Some(v) => match v.as_f64().filter(|n| *n > 0.0 && *n <= 64.0) {
                Some(n) => background.size = Size::Scale(n as f32),
                None => said(format!("size {v}: a number above 0, at most 64")),
            },
        }
        match fields.get("repeat") {
            None => {}
            Some(Value::Bool(b)) => background.repeat = [*b; 2],
            Some(Value::String(s)) => match s.trim().to_ascii_lowercase().as_str() {
                "none" | "no" => background.repeat = [false, false],
                "x" | "across" => background.repeat = [true, false],
                "y" | "down" => background.repeat = [false, true],
                "both" | "yes" => background.repeat = [true, true],
                _ => said(format!("repeat \"{s}\": none, x, y or both")),
            },
            Some(v) => said(format!("repeat {v}: none, x, y or both")),
        }
        match fields.get("position") {
            None => {}
            Some(Value::String(s)) => {
                let mut at = [0.5; 2];
                let mut fine = true;
                for word in s.split_whitespace() {
                    match word.to_ascii_lowercase().as_str() {
                        "left" => at[0] = 0.0,
                        "right" => at[0] = 1.0,
                        "top" => at[1] = 0.0,
                        "bottom" => at[1] = 1.0,
                        "center" | "centre" | "middle" => {}
                        _ => fine = false,
                    }
                }
                match fine {
                    true => background.position = at,
                    false => said(format!("position \"{s}\": center, or top, bottom, left, right (\"top left\")")),
                }
            }
            Some(v) => said(format!("position {v}: center, or top, bottom, left, right (\"top left\")")),
        }
        for (key, place) in [("fixed", &mut background.fixed), ("smooth", &mut background.smooth)] {
            match fields.get(key) {
                None => {}
                Some(Value::Bool(b)) => *place = *b,
                Some(v) => said(format!("{key} {v}: true or false")),
            }
        }
        match fields.get("opacity") {
            None => {}
            Some(v) => match v.as_f64().filter(|n| (0.0..=1.0).contains(n)) {
                Some(n) => background.opacity = n as f32,
                None => said(format!("opacity {v}: a number from 0 to 1")),
            },
        }
        Some(background)
    }

    /// As a theme file writes it.
    fn to_json(&self) -> String {
        let quote = |s: &str| serde_json::to_string(s).unwrap_or_default();
        let size = match self.size {
            Size::Cover => quote("cover"),
            Size::Contain => quote("contain"),
            Size::Stretch => quote("stretch"),
            Size::Scale(1.0) => quote("original"),
            Size::Scale(s) => s.to_string(),
        };
        let repeat = match self.repeat {
            [false, false] => "none",
            [true, false] => "x",
            [false, true] => "y",
            [true, true] => "both",
        };
        let across = match self.position[0] {
            x if x <= 0.0 => "left",
            x if x >= 1.0 => "right",
            _ => "",
        };
        let down = match self.position[1] {
            y if y <= 0.0 => "top",
            y if y >= 1.0 => "bottom",
            _ => "",
        };
        let position = match (down, across) {
            ("", "") => "center".to_string(),
            (d, "") | ("", d) => d.to_string(),
            (d, a) => format!("{d} {a}"),
        };
        format!(
            "{{\n    \"image\": {},\n    \"size\": {size},\n    \"repeat\": \"{repeat}\",\n    \"position\": \"{position}\",\n    \
             \"fixed\": {},\n    \"opacity\": {},\n    \"smooth\": {}\n  }}",
            quote(&self.image.to_string_lossy()),
            self.fixed,
            self.opacity,
            self.smooth
        )
    }

    /// Its image found from `dir` (the theme file's folder) when the theme
    /// names it there.
    pub fn resolve_in(&mut self, dir: &Path) {
        if self.image.is_relative() {
            self.image = dir.join(&self.image);
        }
    }

    /// Where one copy of a picture `picture` pixels big lies: sized and
    /// placed in `room`. `unit` is what one of its pixels is at "original"
    /// (1, or the zoom when it moves with the patch).
    pub fn placed(&self, picture: Vec2, room: Rect, unit: f32) -> Rect {
        let picture = picture.max(Vec2::splat(1.0));
        let size = match self.size {
            Size::Cover => picture * (room.width() / picture.x).max(room.height() / picture.y),
            Size::Contain => picture * (room.width() / picture.x).min(room.height() / picture.y),
            Size::Stretch => room.size(),
            Size::Scale(s) => picture * s * unit,
        };
        Rect::from_min_size(room.min + (room.size() - size) * Vec2::from(self.position), size)
    }

    /// What of a picture placed at `placed` shows in `view`: where to draw,
    /// and that part of the picture (in its own 0 to 1, past them where it
    /// repeats). None when none of it shows.
    pub fn shown(&self, placed: Rect, view: Rect) -> Option<(Rect, Rect)> {
        let axis = |i: usize| {
            let (p0, p1, v0, v1) = match i {
                0 => (placed.left(), placed.right(), view.left(), view.right()),
                _ => (placed.top(), placed.bottom(), view.top(), view.bottom()),
            };
            let (a, b) = if self.repeat[i] { (v0, v1) } else { (p0.max(v0), p1.min(v1)) };
            (b > a && p1 > p0).then(|| ((a, b), ((a - p0) / (p1 - p0), (b - p0) / (p1 - p0))))
        };
        let (((x0, x1), (u0, u1)), ((y0, y1), (v0, v1))) = (axis(0)?, axis(1)?);
        Some((Rect::from_min_max(pos2(x0, y0), pos2(x1, y1)), Rect::from_min_max(pos2(u0, v0), pos2(u1, v1))))
    }
}

impl Theme {
    /// The family's own: phosphor green on black.
    pub fn matrix() -> Theme {
        Theme { name: MATRIX_NAME.into(), colors: MATRIX, background: None, sketch: 0.0, font: None, font_scale: 1.0 }
    }

    /// Soft and dark: slate greys and a muted sage, low glare.
    pub fn dusk() -> Theme {
        Theme { name: "Dusk".into(), colors: DUSK, background: None, sketch: 0.0, font: None, font_scale: 1.0 }
    }

    /// Light: warm paper and deep teal, for bright rooms.
    pub fn paper() -> Theme {
        Theme { name: "Paper".into(), colors: PAPER, background: None, sketch: 0.0, font: None, font_scale: 1.0 }
    }

    /// Paper, drawn by hand: its outlines wobble as a pencil's do, and its
    /// letters are hand written (`HAND_FONT`); buttons and knob rings in a
    /// darker pencil.
    pub fn sketch() -> Theme {
        let mut colors = PAPER;
        colors[Role::BUTTON_BORDER as usize] = 0x8a7f6eff;
        colors[Role::RING as usize] = 0xa89d8bff;
        Theme {
            name: "Sketch".into(),
            colors,
            background: None,
            sketch: 1.0,
            font: Some(PathBuf::from(HAND_FONT)),
            font_scale: 1.0,
        }
    }

    /// The built-in themes, the family's first.
    pub fn built_in() -> [Theme; 4] {
        [Theme::matrix(), Theme::dusk(), Theme::paper(), Theme::sketch()]
    }

    /// The built-in theme called `name` (any case; Matrix also by its old
    /// names).
    pub fn built_in_named(name: &str) -> Option<Theme> {
        let name = name.trim();
        let old = OLD_MATRIX_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name));
        let name = if old { MATRIX_NAME } else { name };
        Theme::built_in().into_iter().find(|t| t.name.eq_ignore_ascii_case(name))
    }

    /// A built-in theme's one-line description; none for others.
    pub fn about(&self) -> Option<&'static str> {
        match self.name.as_str() {
            MATRIX_NAME if self.colors == MATRIX => Some("The Oroboro look: phosphor green on black."),
            "Dusk" if self.colors == DUSK => Some("Soft and dark: slate greys and a muted sage, easy on the eyes."),
            "Paper" if self.colors == PAPER => Some("Light: warm paper and deep teal, for bright rooms."),
            "Sketch" if *self == Theme::sketch() => Some("Paper, drawn by hand: pencil lines and hand-written letters."),
            _ => None,
        }
    }

    pub fn color(&self, role: Role) -> Color32 {
        unpack(self.colors[role as usize])
    }

    pub fn set_color(&mut self, role: Role, color: Color32) {
        let [r, g, b, a] = color.to_srgba_unmultiplied();
        self.colors[role as usize] = u32::from_be_bytes([r, g, b, a]);
    }

    /// Dark (a dark window), so egui's own widgets take their dark look.
    pub fn is_dark(&self) -> bool {
        luminance(self.color(Role::WINDOW)) < 0.4
    }

    /// A theme file: `{"name": "…", "base": "Dusk", "colors": {"accent":
    /// "#88c8ae", …}}`, and maybe a `background` (`Background`). Colours
    /// are `#rrggbb` or `#rrggbbaa`; the ones it doesn't give are the
    /// base's (Matrix when it names none). Returns the theme and what was
    /// wrong in the file but could be left out (an unknown role or base, a
    /// colour that isn't one, a background's setting); an error when it
    /// isn't a theme at all.
    pub fn from_json(text: &str) -> Result<(Theme, Vec<String>), String> {
        let file: File = serde_json::from_str(text).map_err(|e| format!("not a theme: {e}"))?;
        let mut problems = Vec::new();
        let base = match file.base.as_deref().map(str::trim) {
            None | Some("") => Theme::matrix(),
            Some(name) => Theme::built_in_named(name).unwrap_or_else(|| {
                problems.push(format!("no built-in theme \"{name}\" to start from: Matrix instead"));
                Theme::matrix()
            }),
        };
        let name = file.name.trim();
        if name.is_empty() {
            return Err("a theme needs a name".into());
        }
        let mut theme = Theme { name: name.to_string(), colors: base.colors, background: None, ..base };
        for (key, value) in &file.colors {
            match (Role::from_key(key), parse_bits(value)) {
                (Some(role), Some(bits)) => theme.colors[role as usize] = bits,
                (None, _) => problems.push(format!("\"{key}\" is no colour of a theme")),
                (Some(_), None) => problems.push(format!("\"{key}\": \"{value}\" isn't a colour (#rrggbb)")),
            }
        }
        theme.background = file.background.as_ref().and_then(|v| Background::from_value(v, &mut problems));
        // hand drawn, 0 to 3 (the base's when it doesn't say), and a font
        // of its own (a file, beside it when the path is relative)
        if let Some(sketch) = file.sketch {
            theme.sketch = sketch.clamp(0.0, 3.0);
        }
        if let Some(font) = file.font.as_deref().map(str::trim).filter(|f| !f.is_empty()) {
            theme.font = Some(PathBuf::from(font));
        }
        if let Some(scale) = file.font_scale {
            theme.font_scale = scale.clamp(0.5, 2.0);
        }
        Ok((theme, problems))
    }

    /// The theme as a file, every colour in it (to start one of your own
    /// from), `base` named when given, and its background.
    pub fn to_json(&self, base: Option<&str>) -> String {
        let quote = |s: &str| serde_json::to_string(s).unwrap_or_default();
        let mut out = format!("{{\n  \"name\": {},\n", quote(&self.name));
        if let Some(base) = base {
            out += &format!("  \"base\": {},\n", quote(base));
        }
        if let Some(background) = &self.background {
            out += &format!("  \"background\": {},\n", background.to_json());
        }
        if self.sketch > 0.0 {
            out += &format!("  \"sketch\": {},\n", self.sketch);
        }
        if let Some(font) = &self.font {
            out += &format!("  \"font\": {},\n", quote(&font.to_string_lossy()));
        }
        if self.font_scale != 1.0 {
            out += &format!("  \"font_scale\": {},\n", self.font_scale);
        }
        out += "  \"colors\": {\n";
        let lines: Vec<String> =
            Role::ALL.iter().map(|&r| format!("    \"{}\": \"{}\"", r.key(), hex_bits(self.colors[r as usize]))).collect();
        out += &lines.join(",\n");
        out += "\n  }\n}\n";
        out
    }
}

/// `#rrggbb` (opaque) or `#rrggbbaa`, any case.
pub fn parse_color(text: &str) -> Option<Color32> {
    parse_bits(text).map(unpack)
}

/// A colour as a theme file writes it: `#rrggbb`, `#rrggbbaa` when not
/// opaque.
pub fn hex(color: Color32) -> String {
    hex_bits(u32::from_be_bytes(color.to_srgba_unmultiplied()))
}

/// A theme file's colour as the theme keeps it (0xRRGGBBAA, alpha not
/// premultiplied, so a file reads back exactly as written).
fn parse_bits(text: &str) -> Option<u32> {
    let digits = text.trim().strip_prefix('#')?;
    if !matches!(digits.len(), 6 | 8) || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let v = u32::from_str_radix(digits, 16).ok()?;
    Some(if digits.len() == 6 { v << 8 | 0xff } else { v })
}

fn hex_bits(v: u32) -> String {
    match v & 0xff {
        0xff => format!("#{:06x}", v >> 8),
        _ => format!("#{v:08x}"),
    }
}

fn unpack(v: u32) -> Color32 {
    let [r, g, b, a] = v.to_be_bytes();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Relative luminance (WCAG), 0 black to 1 white.
pub fn luminance(color: Color32) -> f32 {
    let linear = |c: u8| {
        let c = f32::from(c) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(color.r()) + 0.7152 * linear(color.g()) + 0.0722 * linear(color.b())
}

/// How far apart two colours read (WCAG's contrast ratio, 1 to 21).
pub fn contrast(a: Color32, b: Color32) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

// ---- The current theme ------------------------------------------------------------

/// The theme chosen for the process, and how many times it was chosen.
static CHOSEN: RwLock<Option<Arc<Theme>>> = RwLock::new(None);
static GENERATION: AtomicU64 = AtomicU64::new(0);

thread_local! {
    /// This thread's colours, and the choice they came from.
    static ACTIVE: Cell<[u32; COUNT]> = const { Cell::new(MATRIX) };
    static TAKEN: Cell<u64> = const { Cell::new(0) };
    static NAME: std::cell::RefCell<String> = std::cell::RefCell::new(String::from(MATRIX_NAME));
    static BACKGROUND: std::cell::RefCell<Option<Background>> = const { std::cell::RefCell::new(None) };
    static SKETCH: Cell<f32> = const { Cell::new(0.0) };
    static FONT: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
    static FONT_SCALE: Cell<f32> = const { Cell::new(1.0) };
}

fn color(role: Role) -> Color32 {
    unpack(ACTIVE.with(|a| a.get()[role as usize]))
}

/// Choose `theme` for the process: every window takes it at its next frame
/// (`sync`).
pub fn apply(theme: &Theme) {
    *CHOSEN.write().unwrap_or_else(PoisonError::into_inner) = Some(Arc::new(theme.clone()));
    GENERATION.fetch_add(1, Ordering::SeqCst);
}

/// Take the process's theme on this thread if it changed since the last
/// time; true when it did (the caller then gives egui its colours:
/// `install_visuals`). Call it at the start of every frame.
pub fn sync() -> bool {
    let chosen = GENERATION.load(Ordering::SeqCst);
    if TAKEN.with(|t| t.get()) == chosen {
        return false;
    }
    TAKEN.with(|t| t.set(chosen));
    let theme = CHOSEN.read().unwrap_or_else(PoisonError::into_inner).clone();
    if let Some(theme) = theme {
        apply_here(&theme);
    }
    true
}

/// Wear `theme` on this thread only (pictures, tests).
pub fn apply_here(theme: &Theme) {
    ACTIVE.with(|a| a.set(theme.colors));
    NAME.with(|n| *n.borrow_mut() = theme.name.clone());
    BACKGROUND.with(|b| *b.borrow_mut() = theme.background.clone());
    SKETCH.with(|s| s.set(theme.sketch));
    FONT.with(|f| *f.borrow_mut() = theme.font.clone());
    FONT_SCALE.with(|s| s.set(theme.font_scale));
}

/// The theme this thread wears.
pub fn current() -> Theme {
    Theme {
        name: NAME.with(|n| n.borrow().clone()),
        colors: ACTIVE.with(|a| a.get()),
        background: background(),
        sketch: sketch(),
        font: FONT.with(|f| f.borrow().clone()),
        font_scale: FONT_SCALE.with(|s| s.get()),
    }
}

/// How hand drawn the outlines are in the theme this thread wears (0: not
/// at all; `sketch::roughen`).
pub fn sketch() -> f32 {
    SKETCH.with(|s| s.get())
}

/// The picture behind the patch in the theme this thread wears.
pub fn background() -> Option<Background> {
    BACKGROUND.with(|b| b.borrow().clone())
}

/// The theme chosen for the process (none chosen yet: the default, Dusk).
pub fn chosen() -> Theme {
    CHOSEN.read().unwrap_or_else(PoisonError::into_inner).as_deref().cloned().unwrap_or_else(Theme::dusk)
}

// ---- Drawing helpers ----------------------------------------------------------------

/// Behind every modal card.
pub fn overlay_dim() -> Color32 {
    DIM()
}

/// The accent at `alpha` (0..255), e.g. for a selected border.
pub fn accent_alpha(alpha: u8) -> Color32 {
    let c = ACCENT();
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
}

/// The cable colour numbered `index` (the G2's order; wraps).
pub fn cable(index: usize) -> Color32 {
    const CABLES: [Role; 7] = [
        Role::CABLE_RED,
        Role::CABLE_BLUE,
        Role::CABLE_YELLOW,
        Role::CABLE_ORANGE,
        Role::CABLE_GREEN,
        Role::CABLE_PURPLE,
        Role::CABLE_WHITE,
    ];
    color(CABLES[index % CABLES.len()])
}

/// Noto Sans Light, the family's one font (`assets/noto-sans`: the SIL Open
/// Font License, its file beside it).
pub const NOTO_SANS_LIGHT: &[u8] = include_bytes!("../assets/noto-sans/NotoSans-Light.ttf");

/// The family's one font, Noto Sans Light, at `size` points.
pub fn font(size: f32) -> FontId {
    FontId::proportional(size)
}

/// Set the family's font, and the theme's colours, on `ctx` (and with
/// fingers egui's own sizes for them: `touch::install`).
pub fn install(ctx: &egui::Context) {
    install_fonts(ctx);
    install_visuals(ctx);
    crate::touch::install(ctx);
}

/// The fonts of the theme this thread wears, on `ctx`: Noto Sans Light,
/// and before it the theme's own font when it has one (`HAND_FONT` or a
/// file; Noto gives what that hasn't), at its `font_scale`. Only when they
/// changed. The layouts go on measuring Noto's widths
/// (`widgets::text_width`): a text drawn wider than its room is made
/// smaller to fit, as ever.
pub fn install_fonts(ctx: &egui::Context) {
    let wanted = (FONT.with(|f| f.borrow().clone()), FONT_SCALE.with(|s| s.get()).to_bits());
    let id = egui::Id::new("oroboro-theme-font");
    if ctx.data(|d| d.get_temp::<(Option<PathBuf>, u32)>(id)).as_ref() == Some(&wanted) {
        return;
    }
    ctx.data_mut(|d| d.insert_temp(id, wanted.clone()));
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("noto-sans-light".into(), Arc::new(FontData::from_static(NOTO_SANS_LIGHT)));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "noto-sans-light".into());
    if let Some(data) = wanted.0.as_deref().and_then(font_data) {
        let tweak = egui::FontTweak { scale: f32::from_bits(wanted.1), ..Default::default() };
        fonts.font_data.insert("theme".into(), Arc::new(data.tweak(tweak)));
        fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "theme".into());
    }
    ctx.set_fonts(fonts);
}

/// A theme's font: the one built in by its name, else a file's, read once
/// (none if it can't be read).
fn font_data(font: &Path) -> Option<FontData> {
    if is_built_in_font(font) {
        return Some(FontData::from_static(HAND_FONT_DATA));
    }
    static READ: Mutex<BTreeMap<PathBuf, Arc<Vec<u8>>>> = Mutex::new(BTreeMap::new());
    let mut read = READ.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(bytes) = read.get(font) {
        return Some(FontData::from_owned(bytes.to_vec()));
    }
    let bytes = Arc::new(std::fs::read(font).ok()?);
    read.insert(font.to_path_buf(), bytes.clone());
    Some(FontData::from_owned(bytes.to_vec()))
}

/// Give egui's own widgets (text fields, menus, tooltips, scroll bars) the
/// colours of the theme this thread wears, and its fonts.
pub fn install_visuals(ctx: &egui::Context) {
    install_fonts(ctx);
    let dark = current().is_dark();
    let mut visuals = if dark { egui::Visuals::dark() } else { egui::Visuals::light() };
    visuals.panel_fill = WINDOW();
    visuals.window_fill = PANEL();
    visuals.window_stroke = Stroke::new(1.0, PANEL_BORDER());
    visuals.extreme_bg_color = WELL();
    visuals.faint_bg_color = LIST();
    visuals.override_text_color = Some(TEXT());
    visuals.selection.bg_fill = SELECTED();
    visuals.selection.stroke = Stroke::new(1.0, ACCENT());
    visuals.hyperlink_color = ACCENT();
    visuals.warn_fg_color = WARNING();
    visuals.error_fg_color = ERROR();
    for (widgets, fill, border) in [
        (&mut visuals.widgets.noninteractive, PANEL(), PANEL_BORDER()),
        (&mut visuals.widgets.inactive, BUTTON(), BUTTON_BORDER()),
        (&mut visuals.widgets.hovered, BUTTON_HOVER(), BUTTON_HOVER_BORDER()),
        (&mut visuals.widgets.active, BUTTON_HOVER(), ACCENT()),
        (&mut visuals.widgets.open, BUTTON_HOVER(), BUTTON_HOVER_BORDER()),
    ] {
        (widgets.bg_fill, widgets.weak_bg_fill) = (fill, fill);
        widgets.bg_stroke = Stroke::new(1.0, border);
    }
    ctx.set_visuals(visuals);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every built-in theme reads well: text, values and knob labels on
    /// the panels, the accent against the window.
    #[test]
    fn the_built_in_themes_read_well() {
        for theme in Theme::built_in() {
            let c = |r| theme.color(r);
            for (what, fore, back, least) in [
                ("text", Role::TEXT, Role::PANEL, 7.0),
                ("values", Role::VALUE, Role::PANEL, 7.0),
                ("labels", Role::LABEL, Role::PANEL, 4.3),
                ("the accent", Role::ACCENT, Role::WINDOW, 4.5),
                ("errors", Role::ERROR, Role::PANEL, 4.4),
            ] {
                let ratio = contrast(c(fore), c(back));
                assert!(ratio >= least, "{}: {what} at {ratio:.2}:1", theme.name);
            }
            assert!(theme.about().is_some());
        }
        assert!(Theme::matrix().is_dark() && Theme::dusk().is_dark() && !Theme::paper().is_dark());
        assert_eq!(Theme::matrix().name, "Matrix");
    }

    /// A theme file names a base and the colours it changes; what's wrong
    /// in it is told and left out; no name, or no JSON, is no theme.
    #[test]
    fn a_theme_file_changes_its_bases_colours() {
        let text = r##"{"name": " Mine ", "base": "dusk", "colors": {"accent": "#FF8800", "shadow": "#00000040",
                       "sparkle": "#ffffff", "text": "blue"}}"##;
        let (theme, problems) = Theme::from_json(text).unwrap();
        assert_eq!(theme.name, "Mine");
        assert_eq!(theme.color(Role::ACCENT), Color32::from_rgb(0xff, 0x88, 0x00));
        assert_eq!(theme.color(Role::SHADOW).a(), 0x40);
        assert_eq!(theme.color(Role::TEXT), Theme::dusk().color(Role::TEXT), "a colour that isn't one");
        assert_eq!(theme.color(Role::PANEL), Theme::dusk().color(Role::PANEL), "the base's");
        assert_eq!(problems.len(), 2, "{problems:?}");
        // a base that isn't there: Matrix, and said; its old names still
        // name it
        let (theme, problems) = Theme::from_json(r#"{"name": "X", "base": "Neon"}"#).unwrap();
        assert_eq!((theme.color(Role::PANEL), problems.len()), (Theme::matrix().color(Role::PANEL), 1));
        let (theme, problems) = Theme::from_json(r#"{"name": "X", "base": "Phosphor"}"#).unwrap();
        assert_eq!((theme.color(Role::ACCENT), problems.len()), (Theme::matrix().color(Role::ACCENT), 0));
        for name in [" matrix ", "The Matrix", "phosphor"] {
            assert_eq!(Theme::built_in_named(name).map(|t| t.name), Some("Matrix".into()), "{name:?}");
        }
        assert!(Theme::from_json(r#"{"colors": {}}"#).is_err() && Theme::from_json("nope").is_err());
        // written out and read back: the same
        let dusk = Theme::dusk();
        let (again, problems) = Theme::from_json(&dusk.to_json(Some("Dusk"))).unwrap();
        assert!(problems.is_empty());
        assert_eq!(again, dusk);
        assert_eq!((parse_bits("#12345678").map(hex_bits), parse_bits("12345")), (Some("#12345678".into()), None));
        assert_eq!(parse_color("#FFFFFF").map(hex), Some("#ffffff".into()));
    }

    /// Drawn by hand: a theme file's `sketch`, `font` and `font_scale` read
    /// (held to their ranges), the base's when it doesn't say, written out
    /// and read back the same; Sketch is Paper drawn by hand in the built-in
    /// hand font, named by its name, not a file.
    #[test]
    fn a_theme_can_be_drawn_by_hand() {
        let (theme, problems) = Theme::from_json(
            r#"{"name": "Mine", "base": "Paper", "sketch": 5, "font": "hand.ttf", "font_scale": 1.3}"#,
        )
        .unwrap();
        assert!(problems.is_empty(), "{problems:?}");
        assert_eq!((theme.sketch, theme.font_scale), (3.0, 1.3), "sketch held to 3");
        assert_eq!(theme.font.as_deref(), Some(Path::new("hand.ttf")));
        let (again, _) = Theme::from_json(&theme.to_json(Some("Paper"))).unwrap();
        assert_eq!(again, theme);
        // the base's when it doesn't say
        let (on_sketch, _) = Theme::from_json(r#"{"name": "Mine", "base": "Sketch"}"#).unwrap();
        assert_eq!((on_sketch.sketch, on_sketch.font.clone(), on_sketch.font_scale), (1.0, Some(PathBuf::from(HAND_FONT)), 1.0));
        assert_eq!(Theme::paper().sketch, 0.0, "the others aren't");
        let sketch = Theme::built_in_named("sketch").expect("built in");
        assert!(is_built_in_font(sketch.font.as_deref().unwrap()) && !is_built_in_font(Path::new("hand.ttf")));
        assert!(font_data(Path::new(HAND_FONT)).is_some() && font_data(Path::new("no such font.ttf")).is_none());
        let (again, _) = Theme::from_json(&sketch.to_json(None)).unwrap();
        assert_eq!(again, sketch, "written out whole, read back the same");
        // the thread wears it: its sketch, its font
        apply_here(&sketch);
        assert_eq!((super::sketch(), current().font.as_deref()), (1.0, Some(Path::new(HAND_FONT))));
        apply_here(&Theme::matrix());
        assert_eq!(super::sketch(), 0.0);
    }

    /// A background: its settings read, what's wrong said and left at the
    /// default, written out and read back the same; sized, placed and cut
    /// to what shows.
    #[test]
    fn a_theme_has_a_picture_behind_the_patch() {
        let text = r##"{"name": "Snow", "base": "Dusk", "background": {"image": "snow.png", "size": "contain",
                       "repeat": "x", "position": "top right", "fixed": false, "opacity": 0.5, "smooth": false}}"##;
        let (theme, problems) = Theme::from_json(text).unwrap();
        assert!(problems.is_empty(), "{problems:?}");
        let b = theme.background.clone().unwrap();
        assert_eq!((b.size, b.repeat, b.position), (Size::Contain, [true, false], [1.0, 0.0]));
        assert_eq!((b.fixed, b.opacity, b.smooth), (false, 0.5, false));
        let (again, problems) = Theme::from_json(&theme.to_json(Some("Dusk"))).unwrap();
        assert!(problems.is_empty() && again == theme, "{problems:?}");
        // the defaults: covering the canvas, in the middle, fixed
        let (theme, _) = Theme::from_json(r#"{"name": "A", "background": {"image": "a.png"}}"#).unwrap();
        let b = theme.background.unwrap();
        assert_eq!((b.size, b.repeat, b.position, b.fixed, b.opacity, b.smooth), (Size::Cover, [false; 2], [0.5; 2], true, 1.0, true));
        // mistakes: said, the rest kept; no image, no background
        let text = r#"{"name": "B", "background": {"image": "b.png", "size": "huge", "opacity": 2, "shine": 1, "repeat": "x"}}"#;
        let (theme, problems) = Theme::from_json(text).unwrap();
        assert_eq!(problems.len(), 3, "{problems:?}");
        assert_eq!(theme.background.map(|b| (b.size, b.opacity, b.repeat)), Some((Size::Cover, 1.0, [true, false])));
        let (theme, problems) = Theme::from_json(r#"{"name": "C", "background": {"size": 2}}"#).unwrap();
        assert!(theme.background.is_none() && problems.len() == 1);
        let (_, problems) = Theme::from_json(r#"{"name": "C", "background": {"image": "c.png", "size": 2}}"#).unwrap();
        assert!(problems.is_empty());
        // a relative file is found in the theme's folder, a whole one stays
        let mut b = Background::from_value(&serde_json::json!({"image": "d.png"}), &mut Vec::new()).unwrap();
        b.resolve_in(Path::new("themes"));
        assert_eq!(b.image, Path::new("themes").join("d.png"));

        // a 200 × 100 picture in a 100 × 100 room
        let room = Rect::from_min_size(pos2(10.0, 20.0), Vec2::splat(100.0));
        let picture = Vec2::new(200.0, 100.0);
        let placed = |b: &Background| b.placed(picture, room, 2.0);
        assert_eq!(placed(&b), Rect::from_min_size(pos2(-40.0, 20.0), Vec2::new(200.0, 100.0)), "covers, cut at the sides");
        let (at, uv) = b.shown(placed(&b), room).unwrap();
        assert_eq!((at, uv), (room, Rect::from_min_max(pos2(0.25, 0.0), pos2(0.75, 1.0))), "its middle half");
        b.size = Size::Contain;
        assert_eq!(placed(&b), Rect::from_min_size(pos2(10.0, 45.0), Vec2::new(100.0, 50.0)), "fits, in the middle");
        b.position = [0.0, 1.0];
        assert_eq!(placed(&b).min, pos2(10.0, 70.0), "at the bottom");
        b.size = Size::Scale(0.25);
        assert_eq!(placed(&b).size(), Vec2::new(100.0, 50.0), "a quarter, at a zoom of 2");
        b.size = Size::Stretch;
        assert_eq!(placed(&b), room);
        // repeating across: as wide as the view, the picture's 0 to 1 over
        // and over
        b.size = Size::Scale(0.1);
        b.position = [0.0, 0.0];
        b.repeat = [true, false];
        let (at, uv) = b.shown(placed(&b), room).unwrap();
        assert_eq!((at.width(), at.height()), (100.0, 20.0));
        assert_eq!((uv.left(), uv.right(), uv.top(), uv.bottom()), (0.0, 2.5, 0.0, 1.0));
        // out of sight: nothing
        assert!(b.shown(placed(&b).translate(Vec2::new(0.0, 500.0)), room).is_none());
    }

    /// The chosen theme reaches a thread when it syncs, not before; a
    /// thread wears its own until then.
    #[test]
    fn a_thread_takes_the_chosen_theme_when_it_syncs() {
        std::thread::spawn(|| {
            apply_here(&Theme::paper());
            assert_eq!(PANEL(), Theme::paper().color(Role::PANEL));
            assert_eq!(current().name, "Paper");
            apply(&Theme::dusk());
            assert_eq!(PANEL(), Theme::paper().color(Role::PANEL), "before the frame");
            assert!(sync());
            assert_eq!((PANEL(), cable(1)), (Theme::dusk().color(Role::PANEL), Theme::dusk().color(Role::CABLE_BLUE)));
            assert!(!sync(), "nothing new");
            assert_eq!(chosen().name, "Dusk");
        })
        .join()
        .unwrap();
    }
}
