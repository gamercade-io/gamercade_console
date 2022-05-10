use eframe::egui::Ui;
use gamercade_core::Palette;

use crate::editor_data::EditorPalette;

#[derive(Clone, Debug)]
pub struct PaletteList {
    palette_data: Vec<EditorPalette>,
    selected_palette: Option<usize>,
}

impl Default for PaletteList {
    fn default() -> Self {
        Self {
            palette_data: Palette::default_palette_collection()
                .into_iter()
                .enumerate()
                .map(|(index, palette)| EditorPalette {
                    name: format!("Palette {}", index),
                    palette,
                })
                .collect(),
            selected_palette: Default::default(),
        }
    }
}

impl PaletteList {
    // Draws the left side panel which displays the palette list widget
    // and related buttons
    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Palette List");

                // Draws the list of palettes
                ui.group(|ui| {
                    self.palette_data
                        .iter()
                        .enumerate()
                        .for_each(|(index, palette)| {
                            let is_checked = if let Some(checked) = self.selected_palette {
                                checked == index
                            } else {
                                false
                            };

                            if ui.selectable_label(is_checked, &palette.name).clicked() {
                                self.selected_palette = Some(index)
                            };
                        });
                });

                // Draws the buttons
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if ui.button("New").clicked() {
                                let count = self.palette_data.len() + 1;
                                self.palette_data.push(EditorPalette {
                                    name: format!("Palette {}", count),
                                    palette: Palette::default(),
                                })
                            };
                            let btn_delete = ui.button("Delete");

                            if let Some(index) = self.selected_palette {
                                if btn_delete.clicked() {
                                    self.palette_data.remove(index);
                                    self.selected_palette = None;
                                }
                            }
                        });

                        ui.vertical(|ui| {
                            let btn_rename = ui.button("Rename");
                            let btn_duplicate = ui.button("Duplicate");

                            if let Some(index) = self.selected_palette {
                                if btn_rename.clicked() {
                                    // TODO: add Rename button
                                    println!("TODO: Rename palettes");
                                }

                                if btn_duplicate.clicked() {
                                    let mut cloned = self.palette_data[index].clone();
                                    cloned.name = format!("{} Copy", cloned.name);
                                    let new_index = index + 1;
                                    self.palette_data.insert(new_index, cloned);
                                    self.selected_palette = Some(new_index);
                                }
                            };
                        });

                        ui.vertical(|ui| {
                            let btn_up = ui.button("Up");
                            let btn_down = ui.button("Down");

                            if let Some(index) = self.selected_palette {
                                if btn_up.clicked() && index != 0 {
                                    self.palette_data.swap(index, index - 1);
                                    self.selected_palette = Some(index - 1);
                                }

                                if btn_down.clicked() && index != self.palette_data.len() - 1 {
                                    self.palette_data.swap(index, index + 1);
                                    self.selected_palette = Some(index + 1);
                                }
                            }
                        });
                    });
                });
            });
        });
    }

    pub fn get_palette(&mut self) -> Option<&mut Palette> {
        if let Some(index) = self.selected_palette {
            Some(&mut self.palette_data[index].palette)
        } else {
            None
        }
    }
}
