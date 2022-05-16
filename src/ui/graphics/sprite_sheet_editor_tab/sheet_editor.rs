use eframe::{
    egui::{ImageButton, Ui},
    epaint::{ColorImage, Vec2},
};
use gamercade_core::{Palette, SpriteSheet};

#[derive(Debug, Clone, Default)]
pub struct SheetEditor {
    pub selected_sprite: usize,
}

impl SheetEditor {
    pub fn draw(&mut self, ui: &mut Ui, sheet: &mut SpriteSheet, scale: usize, palette: &Palette) {
        //TODO
        let step = sheet.width * sheet.height * 4;
        let mut raw_rgba = Vec::with_capacity(step * sheet.sprites.len());

        ui.group(|ui| {
            ui.label("Sprite Sheet Editor");
            ui.label(format!("Sprite Count: {}", sheet.sprites.len()));

            sheet
                .sprites
                .iter()
                .enumerate()
                .for_each(|(index, sprite)| {
                    let start = step * index;
                    let end = start + step;
                    sprite.data.iter().for_each(|color_index| {
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
                    );

                    ui.add(button);
                });
        });
    }
}
