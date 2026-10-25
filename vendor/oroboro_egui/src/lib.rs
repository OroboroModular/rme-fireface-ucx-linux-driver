//! The Oroboro family look in egui (the family's plugin guide, Part 1, the
//! reference being Oroboro Bass's Vizia editor): a flat dark theme with one
//! accent colour, Noto Sans Light, and the family's controls.
//!
//! Generic, with nothing of any one plugin in it, so a plugin can copy the
//! crate as it is. Layouts are fixed-pixel, as in the family: a plugin
//! computes its rects and hands them to the widgets here. With fingers
//! (`touch`) the widgets are drawn larger, and egui's own too, and each finger
//! turns the control it came down on (`fingers`).

pub mod background;
pub mod fingers;
pub mod markdown;
pub mod sketch;
pub mod snapshot;
pub mod theme;
pub mod touch;
pub mod widgets;

pub use egui;
