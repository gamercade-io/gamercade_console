use std::fmt::Write;
use std::{fmt::Display, str::FromStr};

use eframe::egui::{Slider, TextEdit, Ui};

use crate::editor_data::EditorSpriteSheet;

#[derive(Debug, Clone, Default)]
pub struct SheetSettings {
    buffer: String,
}

impl SheetSettings {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut EditorSpriteSheet, palette_count: u8) {
        //TODO
        ui.group(|ui| {
            ui.label("Sprite Sheet Settings");

            ui.horizontal(|ui| {
                self.entry("Name", ui, &mut sheet.name);

                ui.label("Default Palette");
                ui.add(Slider::new(
                    &mut sheet.sprite_sheet.default_palette.0,
                    0..=palette_count - 1,
                ))
            });

            ui.horizontal(|ui| {
                self.entry("Width", ui, &mut sheet.sprite_sheet.width);
                self.entry("Height", ui, &mut sheet.sprite_sheet.height);
            })
        });
    }

    fn entry<T: FromStr + Display>(&mut self, label: &'static str, ui: &mut Ui, value: &mut T) {
        ui.label(label);

        self.buffer.clear();
        write!(&mut self.buffer, "{}", value).unwrap();
        let widget = TextEdit::singleline(&mut self.buffer);
        let response = ui.add(widget);

        if response.changed() {
            if let Ok(new_val) = self.buffer.parse() {
                *value = new_val
            }
        }
    }
}
