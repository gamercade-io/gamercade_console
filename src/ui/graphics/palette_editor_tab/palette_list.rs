use egui::{Color32, Image, TextureId, Ui, Vec2};

use crate::editor_data::{EditorGraphicsData, EditorPalette};
use gamercade_core::Palette;

#[derive(Clone, Debug, Default)]
pub struct PaletteList {
    pub selected_palette: usize,
}

impl PaletteList {
    // Draws the left side panel which displays the palette list widget
    // and related buttons
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        texture_id: TextureId,
        data: &mut EditorGraphicsData,
    ) {
        let index = self.selected_palette;

        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label(format!("Palette List: {}/256", data.palettes.len()));

                // Draws the list of palettes
                ui.group(|ui| {
                    data.palettes
                        .iter()
                        .enumerate()
                        .for_each(|(index, palette)| {
                            ui.horizontal(|ui| {
                                let is_checked = self.selected_palette == index;

                                if ui.selectable_label(is_checked, &palette.name).clicked() {
                                    self.selected_palette = index
                                };

                                // Draws the palette preview
                                ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };
                                palette.palette.colors.iter().for_each(|color| {
                                    let image = Image::new(texture_id, Vec2 { x: 10.0, y: 10.0 })
                                        .tint(Color32::from_rgb(color.r, color.g, color.b));
                                    ui.add(image);
                                })
                            });
                        });
                });

                // Draws the buttons
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if ui.button("New").clicked() {
                                let count = data.palettes.len();

                                if count == u8::MAX as usize + 1 {
                                    println!("Max of 256 Palettes");
                                } else {
                                    data.palettes.push(EditorPalette {
                                        name: format!("Palette {}", count),
                                        palette: Palette::default(),
                                    })
                                }
                            };
                            let btn_delete = ui.button("Delete");

                            if btn_delete.clicked() {
                                if data.palettes.len() != 1 {
                                    data.palettes.remove(index);

                                    if index == data.palettes.len() {
                                        self.selected_palette = index - 1;
                                    };
                                } else {
                                    println!("Can't delete last palette!")
                                }
                            }
                        });

                        ui.vertical(|ui| {
                            let btn_rename = ui.button("Rename");
                            let btn_duplicate = ui.button("Duplicate");

                            if btn_rename.clicked() {
                                // TODO: add Rename button
                                println!("TODO: Rename palettes");
                            }

                            if btn_duplicate.clicked() {
                                if data.palettes.len() == u8::MAX as usize + 1 {
                                    println!("Max of 256 Palettes");
                                } else {
                                    let mut cloned = data.palettes[index].clone();
                                    cloned.name = format!("{} Copy", cloned.name);

                                    let new_index = index + 1;
                                    data.palettes.insert(new_index, cloned);
                                    self.selected_palette = new_index;
                                }
                            };
                        });

                        ui.vertical(|ui| {
                            let btn_up = ui.button("Up");
                            let btn_down = ui.button("Down");

                            if btn_up.clicked() && index != 0 {
                                data.palettes.swap(index, index - 1);
                                self.selected_palette = index - 1;
                            }

                            if btn_down.clicked() && index != data.palettes.len() - 1 {
                                data.palettes.swap(index, index + 1);
                                self.selected_palette = index + 1;
                            }
                        });
                    });
                });
            });
        });
    }
}
