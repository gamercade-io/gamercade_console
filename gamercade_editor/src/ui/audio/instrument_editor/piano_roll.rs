use eframe::{
    egui::{ImageButton, TextureFilter, Ui},
    epaint::{Color32, ColorImage, TextureHandle, Vec2},
};
use gamercade_audio::{NoteColor, NotesIter};

use crate::ui::AudioSyncHelper;

#[derive(Clone, Default)]
pub struct PianoRoll {
    default_piano_texture: Option<TextureHandle>,
}

impl PianoRoll {
    pub(crate) fn draw(&mut self, ui: &mut Ui, _sync: &mut AudioSyncHelper) {
        let texture_id = self
            .default_piano_texture
            .get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "default piano texture",
                    ColorImage::from_rgba_unmultiplied([1, 1], &[255, 255, 255, 255]),
                    TextureFilter::Nearest,
                )
            })
            .id();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2 { x: 1.0, y: 0.0 };
            ui.spacing_mut().button_padding = Vec2 { x: 0.0, y: 0.0 };
            let notes_iter = NotesIter::default();

            notes_iter.for_each(|(note, octave)| {
                let color = match note.get_key_color() {
                    NoteColor::White => Color32::WHITE,
                    NoteColor::Black => Color32::DARK_GRAY,
                };

                let button_top =
                    ImageButton::new(texture_id, Vec2 { x: 12.0, y: 32.0 }).tint(color);
                if ui.add(button_top).clicked() {
                    println!("clicked: {:?} {:?}", note, octave);
                };
            });
        });
    }
}
