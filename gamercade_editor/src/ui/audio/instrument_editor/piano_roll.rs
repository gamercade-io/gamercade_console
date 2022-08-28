use eframe::{
    egui::{ImageButton, TextureFilter, Ui},
    epaint::{Color32, ColorImage, TextureHandle, Vec2},
};
use gamercade_audio::{NoteColor, NotesIter, TOTAL_NOTES_COUNT};

use crate::ui::AudioSyncHelper;

#[derive(Clone, Default)]
pub struct PianoRoll {
    default_piano_texture: Option<TextureHandle>,
}

const NOTE_SPACING: f32 = 1.0;
const TOP_KEY_SIZE: Vec2 = Vec2::new(12.0, 32.0);
const BOTTOM_KEY_SIZE: Vec2 = Vec2::new(
    (((TOP_KEY_SIZE.x + NOTE_SPACING) * TOTAL_NOTES_COUNT as f32) - (NOTE_SPACING * 56.0)) / 56.0,
    24.0,
);

impl PianoRoll {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        sync: &mut AudioSyncHelper,
        selected_instrument: usize,
    ) {
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

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2 {
                x: NOTE_SPACING,
                y: 0.0,
            };
            ui.spacing_mut().button_padding = Vec2 { x: 0.0, y: 0.0 };

            ui.horizontal(|ui| {
                let all_notes_iter = NotesIter::default().enumerate();

                all_notes_iter.for_each(|(index, (note, _octave))| {
                    let color = match note.get_key_color() {
                        NoteColor::White => Color32::WHITE,
                        NoteColor::Black => Color32::DARK_GRAY,
                    };

                    let button_top = ImageButton::new(texture_id, TOP_KEY_SIZE).tint(color);
                    if ui.add(button_top).clicked() {
                        sync.play_note(index, selected_instrument);
                    };
                });
            });

            ui.spacing_mut().item_spacing = Vec2 {
                x: NOTE_SPACING,
                y: 0.0,
            };
            ui.horizontal(|ui| {
                let mut white_notes_iter = NotesIter::default().enumerate();

                for (index, (note, _octave)) in white_notes_iter.by_ref() {
                    if note.get_key_color() == NoteColor::White {
                        let button_bottom = ImageButton::new(texture_id, BOTTOM_KEY_SIZE);

                        if ui.add(button_bottom).clicked() {
                            sync.play_note(index, selected_instrument);
                        };
                    }
                }
            })
        });
    }
}
