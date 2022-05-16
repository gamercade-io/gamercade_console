use egui::{ColorImage, Image, Ui, Vec2};
use gamercade_core::{ColorIndex, Palette, SpriteSheet};

#[derive(Clone, Debug, Default)]
pub struct SpritePreview {}

impl SpritePreview {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        current_palette: &Palette,
        preview_palette: &Palette,
        sprite_sheet: &SpriteSheet,
        sprite_index: usize,
        scale: usize,
    ) {
        // TODO: Write this!
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview: ");

                let sprite = &sprite_sheet.sprites[sprite_index];

                // First Image
                add_image(
                    ui,
                    "Current:",
                    sprite_sheet,
                    &sprite.data,
                    current_palette,
                    scale,
                );

                // Second Image
                add_image(
                    ui,
                    "Preview:",
                    sprite_sheet,
                    &sprite.data,
                    preview_palette,
                    scale,
                );
            });
        });
    }
}

fn add_image(
    ui: &mut Ui,
    label: &'static str,
    sheet: &SpriteSheet,
    sprite: &[ColorIndex],
    palette: &Palette,
    scale: usize,
) {
    let mut raw_rgba = Vec::with_capacity(sheet.width * sheet.height * 4 * 2);

    sprite.iter().for_each(|color_index| {
        let rgba = palette[*color_index].into_pixel_data();
        raw_rgba.extend(rgba);
    });

    let image = ColorImage::from_rgba_unmultiplied([sheet.width, sheet.height], &raw_rgba);

    let image = ui.ctx().load_texture(label, image);

    ui.label(label);
    ui.add(Image::new(
        &image,
        Vec2 {
            x: (sheet.width * scale) as f32,
            y: (sheet.height * scale) as f32,
        },
    ));
}
