use eframe::egui::{Button, Ui};

use gamercade_fs::EditorSpriteSheet;

use super::typed_text_entry;

#[derive(Debug, Clone, Default)]
pub struct SheetSettings {
    editable: EditState,
    buffer: String,
}

impl SheetSettings {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut EditorSpriteSheet) {
        ui.group(|ui| {
            let (is_editable, name, width, height) = match &mut self.editable {
                EditState::Off => {
                    let inner = &mut sheet.sprite_sheet;
                    (false, &mut sheet.name, &mut inner.width, &mut inner.height)
                }
                EditState::On(ref mut p) => (true, &mut p.name, &mut p.width, &mut p.height),
            };

            ui.label("Sprite Sheet Settings");

            ui.horizontal(|ui| {
                typed_text_entry(&mut self.buffer, is_editable, "Name", ui, name);
            });

            ui.horizontal(|ui| {
                typed_text_entry(&mut self.buffer, is_editable, "Width", ui, width);
                typed_text_entry(&mut self.buffer, is_editable, "Height", ui, height);
            });

            ui.horizontal(|ui| {
                if ui.add_enabled(!is_editable, Button::new("Edit")).clicked() {
                    self.editable = EditState::from_editor_sheet(sheet);
                }

                if ui.add_enabled(is_editable, Button::new("Apply")).clicked() {
                    self.update_sheet(sheet);
                }

                if ui.add_enabled(is_editable, Button::new("Cancel")).clicked() {
                    self.editable = EditState::Off;
                }
            })
        });
    }

    fn update_sheet(&mut self, sheet: &mut EditorSpriteSheet) {
        match &self.editable {
            EditState::Off => panic!("Tried to call update_sheet with EditState::Off"),
            EditState::On(update) => {
                let prev_dimensions = (sheet.sprite_sheet.width, sheet.sprite_sheet.height);
                let new_dimensions = (update.width, update.height);

                if prev_dimensions != new_dimensions {
                    sheet
                        .sprite_sheet
                        .resize(new_dimensions.0, new_dimensions.1);
                }

                sheet.name = update.name.clone();
            }
        }

        self.editable = EditState::Off;
    }
}

#[derive(Default, Clone, Debug)]
struct EditableSettings {
    name: String,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
enum EditState {
    Off,
    On(EditableSettings),
}

impl Default for EditState {
    fn default() -> Self {
        Self::Off
    }
}

impl EditState {
    fn from_editor_sheet(other: &EditorSpriteSheet) -> Self {
        Self::On(EditableSettings {
            name: other.name.clone(),
            width: other.sprite_sheet.width,
            height: other.sprite_sheet.height,
        })
    }
}
