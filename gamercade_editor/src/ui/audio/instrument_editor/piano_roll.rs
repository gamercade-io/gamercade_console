use eframe::{
    egui::{ImageButton, Key, TextureFilter, Ui},
    epaint::{Color32, ColorImage, TextureHandle, Vec2},
};
use gamercade_audio::{NoteColor, NotesIter, TOTAL_NOTES_COUNT};

use crate::ui::AudioSyncHelper;

#[derive(Clone, Default)]
pub struct PianoRoll {
    default_piano_texture: Option<TextureHandle>,

    bottom_note_index: usize,
    prev_keys: [bool; KEYBOARD_KEY_COUNT],
    key_channels: [Option<usize>; KEYBOARD_KEY_COUNT],
}

const KEYBOARD_KEY_COUNT: usize = 24;

const NOTE_SPACING: f32 = 1.0;
const TOP_KEY_SIZE: Vec2 = Vec2::new(12.0, 32.0);
const BOTTOM_KEY_SIZE: Vec2 = Vec2::new(
    (((TOP_KEY_SIZE.x + NOTE_SPACING) * TOTAL_NOTES_COUNT as f32) - (NOTE_SPACING * 56.0)) / 56.0,
    24.0,
);

const KEYS: &[Key; KEYBOARD_KEY_COUNT] = &[
    Key::Z,
    Key::S,
    Key::X,
    Key::D,
    Key::C,
    Key::V,
    Key::G,
    Key::B,
    Key::H,
    Key::N,
    Key::J,
    Key::M,
    Key::Q,
    Key::Num2,
    Key::W,
    Key::Num3,
    Key::E,
    Key::R,
    Key::Num5,
    Key::T,
    Key::Num6,
    Key::Y,
    Key::Num7,
    Key::U,
];

impl PianoRoll {
    fn key_in_keyboard_range(&self, index: usize) -> bool {
        index >= self.bottom_note_index && index < self.bottom_note_index + KEYBOARD_KEY_COUNT
    }

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

        let input = ui.input();
        let next_keys = std::array::from_fn(|index| input.key_down(KEYS[index]));

        self.prev_keys
            .iter()
            .zip(next_keys.iter())
            .enumerate()
            .for_each(|(index, (prev, next))| {
                if prev != next {
                    if *next {
                        let assigned_channel =
                            sync.play_note(index + self.bottom_note_index, selected_instrument);
                        self.key_channels[index] = Some(assigned_channel);
                    } else {
                        if let Some(assigned_channel) = self.key_channels[index] {
                            sync.stop_note(assigned_channel);
                        } else {
                            println!("Err: Released key for an unknown note!")
                        }
                    }
                }
            });

        self.prev_keys = next_keys;
        drop(input);

        if ui.button("LEFT").clicked() {
            if self.bottom_note_index > 0 {
                self.bottom_note_index -= 12
            };
        };

        // Draw the actual piano keys for clicking
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2 {
                x: NOTE_SPACING,
                y: 0.0,
            };
            ui.spacing_mut().button_padding = Vec2 { x: 0.0, y: 0.0 };

            ui.horizontal(|ui| {
                let all_notes_iter = NotesIter::default().enumerate();

                all_notes_iter.for_each(|(index, (note, _octave))| {
                    let color = if self.key_in_keyboard_range(index) {
                        match note.get_key_color() {
                            NoteColor::White => Color32::WHITE,
                            NoteColor::Black => Color32::DARK_GRAY,
                        }
                    } else {
                        match note.get_key_color() {
                            NoteColor::White => Color32::LIGHT_GRAY,
                            NoteColor::Black => Color32::BLACK,
                        }
                    };

                    let button_top = ImageButton::new(texture_id, TOP_KEY_SIZE).tint(color);
                    if ui.add(button_top).clicked() {
                        sync.trigger_note(index, selected_instrument);
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
                        let tint = if self.key_in_keyboard_range(index) {
                            Color32::WHITE
                        } else {
                            Color32::LIGHT_GRAY
                        };

                        let button_bottom =
                            ImageButton::new(texture_id, BOTTOM_KEY_SIZE).tint(tint);

                        if ui.add(button_bottom).clicked() {
                            sync.trigger_note(index, selected_instrument);
                        };
                    }
                }
            })
        });

        if ui.button("RIGHT").clicked() {
            if self.bottom_note_index < TOTAL_NOTES_COUNT - KEYBOARD_KEY_COUNT {
                self.bottom_note_index += 12
            };
        };
    }
}
