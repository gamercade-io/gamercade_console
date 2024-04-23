use eframe::egui::{
    self, ColorImage, Image, ImageButton, ScrollArea, TextureFilter, TextureHandle, TextureOptions,
    TextureWrapMode, Ui, Vec2,
};

use super::palette_to_map;
use crate::ui::import_many_images_dialog;
use gamercade_core::{ColorIndex, Palette, SpriteIndex, SpriteSheet};

#[derive(Clone, Default)]
pub struct SheetEditor {
    pub selected_sprite: SpriteIndex,
    raw_rgba_buffer: Vec<u8>,
    texture_handles: Vec<TextureHandle>,
}

impl SheetEditor {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut SpriteSheet, scale: f32, palette: &Palette) {
        let step = sheet.width * sheet.height * 4;
        self.raw_rgba_buffer.clear();

        ui.group(|ui| {
            ui.label("Sprite Sheet Editor");
            ui.label(format!("Sprite Count: {}", sheet.count));

            ui.expand_to_include_y(600.0);

            // Draws all the sprites
            ScrollArea::both().show(ui, |ui| {
                ui.group(|ui| {
                    egui::Grid::new("sprite_sheet_editor_grid").show(ui, |ui| {
                        sheet
                            .iter_sprites()
                            .enumerate()
                            .for_each(|(index, sprite)| {
                                if index != 0 && index % 8 == 0 {
                                    ui.end_row();
                                }

                                let start = step * index;
                                let end = start + step;
                                let index = SpriteIndex(index as u8);

                                sprite.iter().for_each(|color_index| {
                                    let rgba = palette[*color_index].into_pixel_data();
                                    self.raw_rgba_buffer.extend(rgba);
                                });

                                let rgb = ColorImage::from_rgba_unmultiplied(
                                    [sheet.width, sheet.height],
                                    &self.raw_rgba_buffer[start..end],
                                );

                                let image = match self.texture_handles.get_mut(index.0 as usize) {
                                    Some(handle) => {
                                        handle.set(
                                            rgb,
                                            TextureOptions {
                                                magnification: TextureFilter::Nearest,
                                                minification: TextureFilter::Nearest,
                                                wrap_mode: TextureWrapMode::ClampToEdge,
                                            },
                                        );
                                        handle
                                    }
                                    None => {
                                        self.texture_handles.push(ui.ctx().load_texture(
                                            format!("sprite_{}", index.0),
                                            rgb,
                                            TextureOptions {
                                                magnification: TextureFilter::Nearest,
                                                minification: TextureFilter::Nearest,
                                                wrap_mode: TextureWrapMode::ClampToEdge,
                                            },
                                        ));
                                        &mut self.texture_handles[index.0 as usize]
                                    }
                                };

                                let button =
                                    ImageButton::new(Image::new(&*image).fit_to_exact_size(Vec2 {
                                        x: sheet.width as f32 * scale,
                                        y: sheet.height as f32 * scale,
                                    }))
                                    .selected(self.selected_sprite == index);

                                if ui.add(button).clicked() {
                                    self.selected_sprite = index;
                                };
                            });
                    });
                });
            });

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("New").clicked() {
                            sheet.new_empty(self.selected_sprite);
                            self.selected_sprite = SpriteIndex(self.selected_sprite.0 + 1);
                        }

                        if ui.button("Copy").clicked() {
                            sheet.duplicate(self.selected_sprite);
                            self.selected_sprite = SpriteIndex(self.selected_sprite.0 + 1);
                        }

                        // TODO: Add this back later once we implement this feature
                        // if ui.button("Color Swap").clicked() {
                        //     println!("TODO: Color Swap")
                        // }

                        let index = self.selected_sprite.0;

                        if ui.button("Move Left").clicked() {
                            if index != 0 {
                                self.swap_sprites(sheet, index - 1);
                                self.selected_sprite.0 -= 1;
                            } else {
                                println!("Can't move the first sprite left.")
                            }
                        }

                        if ui.button("Move Right").clicked() {
                            if index != sheet.count.saturating_sub(1) {
                                self.swap_sprites(sheet, index);
                                self.selected_sprite.0 += 1;
                            } else {
                                println!("Can't move the last sprite right.");
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Delete").clicked() {
                            if sheet.count != 1 {
                                sheet.delete_sprite(self.selected_sprite);

                                if self.selected_sprite.0 == sheet.count {
                                    self.selected_sprite.0 -= 1;
                                };
                            } else {
                                println!("Can't delete last sprite!")
                            }
                        }

                        if ui.button("Import Frame(s)").clicked() {
                            match try_load_sprites(sheet, palette) {
                                Ok(new_sprites) => new_sprites.iter().for_each(|new_sprite| {
                                    sheet.add_new_sprite(self.selected_sprite, new_sprite);
                                    self.selected_sprite = SpriteIndex(self.selected_sprite.0 + 1);
                                }),
                                Err(e) => println!("{e}"),
                            }
                        }
                    });
                });
            });
        });
    }

    /// Swaps the first index sprite with the next index
    fn swap_sprites(&mut self, sheet: &mut SpriteSheet, first_index: u8) {
        let step = sheet.step();
        let first_index = first_index as usize * step;
        let last_index = first_index + (2 * step);
        let len = last_index - first_index;
        let two_sprites = &mut sheet.sprites[first_index..last_index];
        let (first, second) = two_sprites.split_at_mut(len / 2);
        first.swap_with_slice(second);
    }
}

fn try_load_sprites(
    sheet: &SpriteSheet,
    palette: &Palette,
) -> Result<Vec<Box<[ColorIndex]>>, String> {
    // File opening stuff
    let images = match import_many_images_dialog("Import Sprites...") {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    if images.is_empty() {
        return Err("Returned an empty vector of images".to_string());
    }

    let colors = palette_to_map(palette);
    let mut out_group = Vec::new();

    let no_alpha_color_index = colors
        .iter()
        .find(|(color, _)| color.0[3] == 0)
        .map(|(_, index)| *index);

    for image in images.iter() {
        // Check if dimensions match
        if sheet.width as u32 != image.width() || sheet.height as u32 != image.height() {
            return Err(
                "Imported image width and height don't match the sprite sheet.".to_string(),
            );
        }

        // Build the colors map, and load the sprite
        let mut new_sprite = Vec::with_capacity(image.len());

        for color in image.pixels() {
            if let (Some(no_alpha_color), 0) = (no_alpha_color_index, color.0[3]) {
                new_sprite.push(no_alpha_color)
            } else if let Some(index) = colors.get(color) {
                new_sprite.push(*index)
            } else {
                return Err(format!(
                    "Image contains a color not found in the palette: {color:?}"
                ));
            }
        }

        out_group.push(new_sprite.into_boxed_slice());
    }
    Ok(out_group)
}
