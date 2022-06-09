use egui::{TextureId, Ui};
use hashbrown::HashSet;

use crate::{
    editor_data::{EditorGraphicsData, EditorPalette},
    ui::{draw_palette_preview, import_image_dialog},
};
use gamercade_core::{Color, Palette, PALETTE_COLORS};

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
                                draw_palette_preview(ui, &palette.palette, texture_id);
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
                            let btn_import = ui.button("Import");
                            let btn_export = ui.button("Export");

                            if btn_import.clicked() {
                                if data.palettes.len() == u8::MAX as usize + 1 {
                                    println!("Max of 256 Palettes");
                                } else {
                                    match try_load_palette() {
                                        Ok(loaded) => {
                                            let new_index = index + 1;
                                            data.palettes.insert(new_index, loaded);
                                            self.selected_palette = new_index;
                                        }
                                        Err(e) => println!("{}", e),
                                    }
                                }
                            };

                            if btn_export.clicked() {
                                // TODO: add Export Palette button
                                println!("TODO: Export Palette");
                            }
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

fn try_load_palette() -> Result<EditorPalette, String> {
    // Loading file stuff
    let (image, name) = match import_image_dialog("Import Palette...") {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    // Find each color in the image...
    let mut colors = HashSet::new();

    for pixel in image.pixels() {
        match pixel[3] {
            u8::MAX => {
                if colors.insert(*pixel) {
                    // If there are > the allowed number of colors, it's invalid
                    println!("added: {:?}", pixel);
                }
            }
            0 => continue,
            a => {
                return Err(format!(
                    "Image contains pixel with alpha value {}. Alpha must be 0 or 255",
                    a
                ))
            }
        }
    }

    let color_count = colors.len();
    if color_count > PALETTE_COLORS {
        return Err(format!(
            "Image has {} colors. Maximum allowed is {}.",
            color_count, PALETTE_COLORS
        ));
    }

    // We have a valid palette, so start building it
    let mut palette = Palette {
        colors: [Color::default(); PALETTE_COLORS],
    };

    colors.iter().enumerate().for_each(|(index, rgb)| {
        palette.colors[index] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    });

    Ok(EditorPalette { name, palette })
}
