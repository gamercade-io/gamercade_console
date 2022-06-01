use egui::{ColorImage, ImageButton, Ui, Vec2};
use rfd::FileDialog;

use gamercade_core::{Palette, SpriteIndex, SpriteSheet};

#[derive(Debug, Clone, Default)]
pub struct SheetEditor {
    pub selected_sprite: SpriteIndex,
}

impl SheetEditor {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut SpriteSheet, scale: usize, palette: &Palette) {
        let step = sheet.width * sheet.height * 4;
        let mut raw_rgba = Vec::with_capacity(step * sheet.sprites.len());

        ui.group(|ui| {
            ui.label("Sprite Sheet Editor");
            ui.label(format!("Sprite Count: {}", sheet.count));

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
                            raw_rgba.extend(rgba);
                        });
                        let image = ColorImage::from_rgba_unmultiplied(
                            [sheet.width, sheet.height],
                            &raw_rgba[start..end],
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
                        if let Some(file_path) = FileDialog::new()
                            .set_title("Import...")
                            .set_directory("/")
                            .add_filter(
                                "image",
                                &["png", "jpeg", "gif", "bmp", "ico", "tiff", "tga"],
                            )
                            .pick_file()
                        {
                            println!("TODO: Load file: {:?}", file_path);
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
