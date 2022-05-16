use eframe::egui::Ui;
use gamercade_core::SpriteSheet;

use crate::editor_data::{EditorGraphicsData, EditorSpriteSheet};

#[derive(Debug, Clone, Default)]
pub struct SheetList {
    pub selected_sheet: usize,
}

impl SheetList {
    pub(crate) fn draw(&mut self, ui: &mut Ui, data: &mut EditorGraphicsData) {
        let index = self.selected_sheet;

        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Sprite Sheet List");

                // Draws the list of sheets
                ui.group(|ui| {
                    data.sprite_sheets
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
                            let count = data.sprite_sheets.len() + 1;
                            data.sprite_sheets.push(EditorSpriteSheet {
                                name: format!("Sprite Sheet {}", count),
                                sprite_sheet: SpriteSheet::default(),
                            })
                        };

                        if ui.button("Delete").clicked() {
                            if data.sprite_sheets.len() != 1 {
                                data.sprite_sheets.remove(index);

                                if index == data.sprite_sheets.len() {
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
}
