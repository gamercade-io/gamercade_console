use eframe::egui::Ui;
use gamercade_core::{Palette, SpriteSheet, SpriteSheetIndex};

use crate::ui::import_image_dialog;
use gamercade_fs::EditorSpriteSheet;

use super::sprite_sheet_importer::SpriteSheetImporter;

#[derive(Debug, Clone, Default)]
pub struct SheetList {
    pub selected_sheet: SpriteSheetIndex,

    importer: SpriteSheetImporter,
}

impl SheetList {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut Vec<EditorSpriteSheet>,
        palette: &Palette,
    ) {
        self.importer.draw(ui, data, palette);

        let index = self.selected_sheet;

        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Sprite Sheet List");

                // Draws the list of sheets
                ui.group(|ui| {
                    data.iter().enumerate().for_each(|(index, sheet)| {
                        ui.horizontal(|ui| {
                            let index = SpriteSheetIndex(index as u8);
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
                            let count = data.len() + 1;
                            data.push(EditorSpriteSheet {
                                name: format!("Sprite Sheet {count}"),
                                sprite_sheet: SpriteSheet::default(),
                            })
                        };

                        if ui.button("Import Sprite Sheet").clicked() {
                            match import_image_dialog("Import Sprite Sheet...") {
                                Ok(image) => self.importer.image_buffer = Some(image),
                                Err(e) => println!("{e}"),
                            }
                        };

                        if ui.button("Delete").clicked() {
                            if data.len() != 1 {
                                data.remove(index.0 as usize);

                                if index.0 as usize == data.len() {
                                    self.selected_sheet = SpriteSheetIndex(index.0 - 1);
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
