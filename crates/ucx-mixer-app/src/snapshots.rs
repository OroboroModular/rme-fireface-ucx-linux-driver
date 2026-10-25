//! Snapshots, as TotalMix has them: eight slots, each a whole mix with a name. Storing one copies
//! the mix as it is; recalling one makes it the mix and sends all of it, as far as the reach goes
//! (`MixerUi::send_whole_mix`). They're kept with the window's state (`settings.rs`).

use oroboro_egui::egui::{self, pos2, vec2, Align2, Rect, Ui, Vec2};
use oroboro_egui::theme::*;
use oroboro_egui::widgets::{self as look, ButtonKind};
use serde::{Deserialize, Serialize};
use ucx_mixer::Mixer;

use crate::view::MixerUi;

/// How many slots.
pub const SLOTS: usize = 8;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub name: String,
    pub mix: Mixer,
}

/// The card.
pub const CARD: Vec2 = vec2(520.0, 428.0);
const LINE: f32 = 40.0;

impl MixerUi {
    /// Slot `i` holds the mix as it is now, under `name`.
    pub fn store(&mut self, i: usize, name: String) {
        self.snapshots[i] = Some(Snapshot { name, mix: self.mixer.clone() });
    }

    /// Slot `i`'s mix becomes the mix, and goes out whole as far as the reach goes; what the unit
    /// keeps itself (output levels, preamp gains) stays as it is now. False for an empty slot.
    pub fn recall(&mut self, i: usize) -> bool {
        let Some(snapshot) = self.snapshots[i].clone() else { return false };
        let mut mix = snapshot.mix.clamped();
        for (o, now) in mix.outputs.iter_mut().zip(&self.mixer.outputs) {
            o.level = now.level;
        }
        for (p, now) in mix.preamps.iter_mut().zip(&self.mixer.preamps) {
            p.gain = now.gain;
        }
        self.mixer = mix;
        self.send_whole_mix();
        true
    }
}

/// Draws the card over `area`.
pub fn card(ui: &mut Ui, s: &mut MixerUi, area: Rect) {
    let ctx = ui.ctx().clone();
    look::overlay(&ctx, "ucx-snapshots", area, |ui| {
        let rect = look::card_rect(ui, area, CARD, "ucx-snapshots");
        let body = look::card(ui, rect, "Snapshots");
        for i in 0..SLOTS {
            let y = body.top() + i as f32 * LINE;
            let painter = ui.painter();
            painter.text(pos2(body.left(), y + 14.0), Align2::LEFT_CENTER, format!("{}", i + 1), font(12.0), MUTED());
            // the name: typed here, kept for the next store
            let field = Rect::from_min_size(pos2(body.left() + 24.0, y), vec2(250.0, 28.0));
            let id = egui::Id::new(("ucx-snapshot-name", i));
            let mut name: String = ui.data(|d| d.get_temp(id)).unwrap_or_else(|| match &s.snapshots[i] {
                Some(snap) => snap.name.clone(),
                None => String::new(),
            });
            let edit = egui::TextEdit::singleline(&mut name)
                .hint_text(format!("Snapshot {}", i + 1))
                .font(egui::FontId::proportional(12.0))
                .vertical_align(egui::Align::Center);
            if ui.put(field, edit).changed() {
                ui.data_mut(|d| d.insert_temp(id, name.clone()));
            }
            let stored = s.snapshots[i].is_some();
            let recall = Rect::from_min_size(pos2(body.left() + 286.0, y), vec2(90.0, 28.0));
            let store = Rect::from_min_size(pos2(body.left() + 384.0, y), vec2(90.0, 28.0));
            if stored && look::button(ui, recall, "Recall", ButtonKind::Primary, false, ("ucx-recall", i)).clicked() {
                s.recall(i);
            }
            if look::button(ui, store, "Store", ButtonKind::Panel, false, ("ucx-store", i)).clicked() {
                let name = if name.trim().is_empty() { format!("Snapshot {}", i + 1) } else { name.trim().to_string() };
                ui.data_mut(|d| d.insert_temp(id, name.clone()));
                s.store(i, name);
            }
        }
        let note = "Recall sends the whole mix, as far as Send to unit and Analog outputs go.";
        let painter = ui.painter();
        painter.text(pos2(body.left(), body.bottom() - 14.0), Align2::LEFT_CENTER, note, font(11.0), MUTED());
        let done = Rect::from_min_size(pos2(body.right() - 100.0, body.bottom() - 28.0), vec2(100.0, 28.0));
        if look::button(ui, done, "Done", ButtonKind::Panel, false, "ucx-snapshots-done").clicked()
            || ui.input(|i| i.key_pressed(egui::Key::Escape))
        {
            s.snapping = false;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::Reach;
    use ucx_mixer::{Send, Source};

    #[test]
    fn store_and_recall() {
        let mut s = MixerUi::new();
        s.change_send(4, Source::Playback(1), Send { level: -60, pan: 0 });
        s.store(2, "Digital test".into());
        s.change_send(4, Source::Playback(1), Send::OFF);
        s.change_send(0, Source::Input(0), Send { level: 0, pan: 0 });
        assert!(!s.recall(0), "an empty slot");
        s.reach = Reach::Digital;
        s.outgoing.clear();
        let held = s.held.len();
        s.mixer.set_output_level(0, -300);
        assert!(s.recall(2));
        assert_eq!(s.mixer.outputs[0].level, Some(-300), "the unit's own level stays as it is");
        assert_eq!(s.mixer.send(4, Source::Playback(1)), Send { level: -60, pan: 0 });
        assert_eq!(s.mixer.send(0, Source::Input(0)), Send::OFF, "the whole mix, the analog part too");
        assert!(!s.outgoing.is_empty() && s.outgoing.iter().all(|w| ucx_mixer::digital_only(*w)));
        assert!(s.held.len() > held, "the analog part held back");
        assert_eq!(s.snapshots[2].as_ref().unwrap().name, "Digital test");
    }
}
