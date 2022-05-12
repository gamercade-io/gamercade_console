use eframe::egui::Ui;
use gamercade_core::SpriteSheet;

use crate::editor_data::EditorSpriteSheet;

#[derive(Debug, Clone)]
pub struct SheetList {
    sheet_data: Vec<EditorSpriteSheet>,
    selected_sheet: usize,
}

impl Default for SheetList {
    fn default() -> Self {
        Self {
            sheet_data: vec![EditorSpriteSheet {
                name: "Sprite Sheet 1".to_string(),
                sprite_sheet: SpriteSheet::default(),
            }],
            selected_sheet: Default::default(),
        }
    }
}

impl SheetList {
    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        let index = self.selected_sheet;

        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Sprite Sheet List");

                // Draws the list of sheets
                ui.group(|ui| {
                    self.sheet_data
                        .iter()
                        .enumerate()
                        .for_each(|(index, sheet)| {
                            ui.horizontal(|ui| {
                                let is_checked = self.selected_sheet == index;

                                if ui.selectable_label(is_checked, &sheet.name).clicked() {
                                    self.selected_sheet = index
                                };
                            });
                        });
                });

                // Draws the buttons
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("New").clicked() {
                            let count = self.sheet_data.len() + 1;
                            self.sheet_data.push(EditorSpriteSheet {
                                name: format!("Sprite Sheet {}", count),
                                sprite_sheet: SpriteSheet::default(),
                            })
                        };

                        if ui.button("Delete").clicked() {
                            if self.sheet_data.len() != 1 {
                                self.sheet_data.remove(index);

                                if index == self.sheet_data.len() {
                                    self.selected_sheet = index - 1;
                                };
                            } else {
                                println!("Can't delete last sheet!")
                            }
                        }
                    });
                });
            });
        });
    }

    pub fn get_sprite_sheet_mut(&mut self) -> &mut EditorSpriteSheet {
        &mut self.sheet_data[self.selected_sheet]
    }
}
