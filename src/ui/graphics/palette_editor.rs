use eframe::{
    egui::{color_picker::show_color, ImageButton, Ui},
    epaint::{Color32, Rect, Rounding, Shape, TextureId, Vec2},
};
use gamercade_core::{Palette, PALETTE_COLORS};

use crate::editor_data::EditorPalette;

#[derive(Debug, Clone)]
pub struct PaletteEditor {
    palette_data: Vec<EditorPalette>,
    selected_palette: Option<usize>,
    selected_color: Option<usize>,
}

impl Default for PaletteEditor {
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
            selected_color: Default::default(),
        }
    }
}

impl PaletteEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.draw_palette_list(ui);
            self.draw_right_side(ui)
        });
    }

    pub fn get_palette(&mut self) -> Option<&mut Palette> {
        if let Some(index) = self.selected_palette {
            Some(&mut self.palette_data[index].palette)
        } else {
            None
        }
    }

    // Draws the left side panel which displays the palette list widget
    // and related buttons
    fn draw_palette_list(&mut self, ui: &mut Ui) {
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

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.draw_palette_viewer(ui);

            ui.horizontal(|ui| {
                self.draw_color_editor(ui);
                self.draw_sprite_preview(ui);
            });
        });
    }

    fn draw_palette_viewer(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Palette Viewer");

            ui.horizontal(|ui| {
                let palette = if let Some(index) = self.selected_palette {
                    Some(&mut self.palette_data[index].palette)
                } else {
                    None
                };

                if let Some(palette) = palette {
                    palette
                        .colors
                        .iter()
                        .enumerate()
                        .for_each(|(index, color)| {
                            // TODO: Get this to work
                            // let selected = if let Some(selected_color) = self.selected_color {
                            //     index == selected_color
                            // } else {
                            //     false
                            // };

                            // let shape = Shape::rect_filled(
                            //     Rect {
                            //         min: [0.0, 0.0].into(),
                            //         max: [32.0, 32.0].into(),
                            //     },
                            //     Rounding::same(0.01),
                            //     Color32::from_rgb(color.r, color.g, color.b),
                            // );

                            // ui.image(shape.texture_id(), Vec2 { x: 32.0, y: 32.0 });

                            // println!("{:?}", shape.texture_id());

                            // // let image_button =
                            // //     ImageButton::new(shape.texture_id(), Vec2 { x: 32.0, y: 32.0 })
                            // //         .selected(selected);

                            // // if ui.add(image_button).clicked() {
                            // //     self.selected_color = Some(index)
                            // // };
                        });
                }
            })
        });
    }

    // Draws the color editor widget
    fn draw_color_editor(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Color Editor");
            });
        });
    }

    fn draw_sprite_preview(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview");
            });
        });
    }
}
