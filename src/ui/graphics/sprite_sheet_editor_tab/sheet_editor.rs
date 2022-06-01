use std::path::PathBuf;

use egui::{ColorImage, ImageButton, Ui, Vec2};
use hashbrown::{HashMap, HashSet};
use image::Rgb;
use rfd::FileDialog;

use gamercade_core::{Color, ColorIndex, Palette, SpriteIndex, SpriteSheet, PALETTE_COLORS};

use crate::editor_data::EditorPalette;

#[derive(Debug, Clone, Default)]
pub struct SheetEditor {
    pub selected_sprite: SpriteIndex,
    raw_rgba_buffer: Vec<u8>,
}

impl SheetEditor {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        sheet: &mut SpriteSheet,
        scale: usize,
        palettes: &mut Vec<EditorPalette>,
        selected_palette: usize,
    ) {
        let step = sheet.width * sheet.height * 4;
        self.raw_rgba_buffer.clear();

        ui.group(|ui| {
            ui.label("Sprite Sheet Editor");
            ui.label(format!("Sprite Count: {}", sheet.count));
            let palette = &palettes[selected_palette].palette;

            // Draws all the sprites
            // TODO: Make this tiled to fill up space correctly
            ui.group(|ui| {
                sheet
                    .iter_sprites()
                    .enumerate()
                    .for_each(|(index, sprite)| {
                        let start = step * index;
                        let end = start + step;
                        let index = SpriteIndex(index as u8);

                        sprite.iter().for_each(|color_index| {
                            let rgba = palette[*color_index].into_pixel_data();
                            self.raw_rgba_buffer.extend(rgba);
                        });
                        let image = ColorImage::from_rgba_unmultiplied(
                            [sheet.width, sheet.height],
                            &self.raw_rgba_buffer[start..end],
                        );
                        let mut image = ui.ctx().load_texture("sprit editor", image);

                        let button = ImageButton::new(
                            &mut image,
                            Vec2 {
                                x: (sheet.width * scale) as f32,
                                y: (sheet.height * scale) as f32,
                            },
                        )
                        .selected(self.selected_sprite == index);

                        if ui.add(button).clicked() {
                            self.selected_sprite = index;
                        };
                    });
            });

            // TODO: add editor buttons:
            // New, Copy, Move Left, Move Right
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

                    if ui.button("Color Swap").clicked() {
                        println!("TODO: Color Swap")
                    }

                    if ui.button("Move Left").clicked() {
                        println!("TODO: Move Left")
                    }

                    if ui.button("Move Right").clicked() {
                        println!("TODO: Move Right")
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Delete").clicked() {
                        println!("TODO: call sheet.delete_sprite()");
                    }

                    if ui.button("Import").clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_title("Import...")
                            .set_directory("/")
                            .add_filter(
                                "image",
                                &["png", "jpeg", "gif", "bmp", "ico", "tiff", "tga"],
                            )
                            .pick_file()
                        {
                            if try_load_sprite(path, sheet, self.selected_sprite, palettes) {
                                self.selected_sprite = SpriteIndex(self.selected_sprite.0 + 1);
                            }
                        }
                    }

                    if ui.button("Export").clicked() {
                        println!("TODO: Export")
                    }
                });
            });
        });
    }
}

fn try_load_sprite(
    path: PathBuf,
    sheet: &mut SpriteSheet,
    index: SpriteIndex,
    palettes: &mut Vec<EditorPalette>,
) -> bool {
    println!("TODO: Load file: {:?}", path);
    let image_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let image = match image::open(path) {
        Ok(image) => image.into_rgb8(),
        Err(e) => {
            println!("Failed to load iamge: {:?}", e);
            return false;
        }
    };

    // Check if dimensions match
    if sheet.width as u32 != image.width() && sheet.height as u32 != image.height() {
        println!("Imported image width and height don't match the sprite sheet.");
        return false;
    }

    // Find each color in the image...
    let mut colors = HashSet::new();

    for pixel in image.pixels() {
        if colors.insert(pixel.clone()) {
            // If there are > the allowed number of colors, it's invalid
            if colors.len() > PALETTE_COLORS {
                println!(
                    "Image has more than the allowed colors of: {}",
                    PALETTE_COLORS
                );
                return false;
            }
        }
    }

    let mut test_map = HashMap::new();
    let mut test_map_keys = HashSet::new();

    // TODO:
    // See if a palette containing all of those colors already exists
    let color_map = if palettes.iter().any(|palette| {
        test_map.clear();
        test_map_keys.clear();
        palette
            .palette
            .colors
            .iter()
            .enumerate()
            .for_each(|(index, color)| {
                let key = Rgb::<u8>([color.r, color.g, color.b]);
                test_map_keys.insert(key);
                test_map.insert(key, ColorIndex(index as u8));
            });
        test_map_keys == colors
    }) {
        // If so use it
        test_map
    } else {
        // otherwise, create a new palette with the new colors
        let mut palette = [Color::default(); PALETTE_COLORS];

        colors.iter().enumerate().for_each(|(index, rgb)| {
            palette[index] = Color {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            }
        });

        palettes.push(EditorPalette {
            name: image_name,
            palette: Palette { colors: palette },
        });

        colors
            .iter()
            .enumerate()
            .map(|(index, rgb)| (*rgb, ColorIndex(index as u8)))
            .collect()
    };

    // Convert the image pixels into color indices
    let sprite = image
        .pixels()
        .map(|pixel| *color_map.get(pixel).unwrap())
        .collect::<Vec<_>>();

    sheet.add_new_sprite(index, &sprite);

    true
}
