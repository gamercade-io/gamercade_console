use std::fmt::Write;
use std::{fmt::Display, str::FromStr};

use egui::{Button, Slider, TextEdit, Ui};

use crate::editor_data::EditorSpriteSheet;

#[derive(Debug, Clone, Default)]
pub struct SheetSettings {
    editable: bool,
    buffer: String,
    preview: EditorSpriteSheet,
}

impl SheetSettings {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut EditorSpriteSheet, palette_count: u8) {
        if !self.editable && self.preview != *sheet {
            self.preview = sheet.clone();
        }

        ui.group(|ui| {
            ui.label("Sprite Sheet Settings");

            ui.horizontal(|ui| {
                entry(
                    &mut self.buffer,
                    self.editable,
                    "Name",
                    ui,
                    &mut self.preview.name,
                );

                ui.label("Default Palette");
                ui.add_enabled(
                    self.editable,
                    Slider::new(
                        &mut self.preview.sprite_sheet.default_palette.0,
                        0..=palette_count - 1,
                    ),
                )
            });

            ui.horizontal(|ui| {
                entry(
                    &mut self.buffer,
                    self.editable,
                    "Width",
                    ui,
                    &mut self.preview.sprite_sheet.width,
                );
                entry(
                    &mut self.buffer,
                    self.editable,
                    "Height",
                    ui,
                    &mut self.preview.sprite_sheet.height,
                );
            });

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(!self.editable, Button::new("Edit"))
                    .clicked()
                {
                    self.editable = true;
                }

                if ui
                    .add_enabled(self.editable, Button::new("Apply"))
                    .clicked()
                {
                    *sheet = self.preview.clone();
                    self.editable = false;
                }

                if ui
                    .add_enabled(self.editable, Button::new("Cancel"))
                    .clicked()
                {
                    self.editable = false;
                }
            })
        });
    }
}

fn entry<T: FromStr + Display>(
    buffer: &mut String,
    editable: bool,
    label: &'static str,
    ui: &mut Ui,
    value: &mut T,
) {
    ui.label(label);

    buffer.clear();
    write!(buffer, "{}", value).unwrap();
    let widget = TextEdit::singleline(buffer);
    let response = ui.add_enabled(editable, widget);

    if response.changed() {
        if let Ok(new_val) = buffer.parse() {
            *value = new_val
        }
    }
}
