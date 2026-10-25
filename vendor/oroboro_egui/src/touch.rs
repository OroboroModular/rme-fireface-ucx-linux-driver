//! Fingers, not a mouse: on a touch screen (iOS) the window's controls are
//! made big enough to press with a finger, and egui's own (menus, lists,
//! fields) too. Layouts ask `touch::on()` as they lay themselves out.

use std::cell::Cell;

use egui::{vec2, Context, FontFamily, FontId, Margin, TextStyle};

thread_local! {
    static HERE: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Whether the window is worked with fingers: on iOS, or where this
/// thread was told so (`set_here`, for tests and pictures).
pub fn on() -> bool {
    HERE.with(Cell::get).unwrap_or(cfg!(target_os = "ios"))
}

/// Fingers (`Some(true)`) or a mouse on this thread, whatever the
/// platform; `None`: the platform's.
pub fn set_here(touch: Option<bool>) {
    HERE.with(|here| here.set(touch));
}

/// `mouse` with a mouse, `finger` with fingers.
pub fn pick<T>(mouse: T, finger: T) -> T {
    if on() {
        finger
    } else {
        mouse
    }
}

/// How tall a control a finger presses is (a button, a selector, a field,
/// a menu's row): Apple asks 44 points of a target, which this and the
/// gaps around it give.
pub const CONTROL: f32 = 40.0;

/// egui's own widgets as fingers want them, on `ctx`: buttons, menu rows
/// and fields `CONTROL` tall, larger text, more room between items.
/// Nothing with a mouse.
pub fn install(ctx: &Context) {
    if !on() {
        return;
    }
    ctx.all_styles_mut(|style| {
        let spacing = &mut style.spacing;
        // (a row: the widget, and the gap under it)
        spacing.interact_size = vec2(44.0, CONTROL - 6.0);
        spacing.item_spacing = vec2(10.0, 6.0);
        spacing.button_padding = vec2(14.0, 8.0);
        spacing.menu_margin = Margin::same(8);
        spacing.icon_width = 22.0;
        spacing.icon_width_inner = 14.0;
        spacing.icon_spacing = 10.0;
        spacing.slider_width = 180.0;
        spacing.combo_height = 320.0;
        spacing.scroll.bar_width = 8.0;
        style.text_styles = [
            (TextStyle::Small, 12.0),
            (TextStyle::Body, 16.0),
            (TextStyle::Button, 16.0),
            (TextStyle::Heading, 21.0),
        ]
        .into_iter()
        .map(|(style, size)| (style, FontId::new(size, FontFamily::Proportional)))
        .chain([(TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace))])
        .collect();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A thread is told fingers or a mouse, whatever the platform; told
    /// nothing, it's the platform's.
    #[test]
    fn a_thread_is_told_fingers_or_a_mouse() {
        set_here(Some(true));
        assert!(on());
        assert_eq!(pick(1, 2), 2);
        set_here(Some(false));
        assert!(!on());
        assert_eq!(pick(1, 2), 1);
        set_here(None);
        assert_eq!(on(), cfg!(target_os = "ios"));
    }

    /// With fingers, egui's buttons and menu rows are a finger tall, and
    /// its text larger; with a mouse, egui's own sizes stay.
    #[test]
    fn fingers_get_taller_rows_and_larger_text() {
        set_here(Some(false));
        let ctx = Context::default();
        install(&ctx);
        let mouse = ctx.global_style().spacing.interact_size.y;
        set_here(Some(true));
        install(&ctx);
        let style = ctx.global_style();
        assert!(style.spacing.interact_size.y + style.spacing.item_spacing.y >= CONTROL && mouse < CONTROL - 6.0);
        assert_eq!(style.text_styles[&TextStyle::Body].size, 16.0);
        set_here(None);
    }
}
