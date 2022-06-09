use egui::{ColorImage, Image, Ui, Vec2};
use gamercade_core::{ColorIndex, Palette, SpriteIndex, SpriteSheet};

#[derive(Clone, Debug, Default)]
pub struct SpritePreview {
    current_buffer: Vec<u8>,
    preview_buffer: Vec<u8>,
}

impl SpritePreview {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        current_palette: &Palette,
        preview_palette: &Palette,
        sprite_sheet: &SpriteSheet,
        sprite_index: SpriteIndex,
        scale: usize,
    ) {
        // TODO: Write this!
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview: ");

                let sprite = &sprite_sheet[sprite_index];

                // First Image
                add_image(
                    ui,
                    "Current:",
                    &mut self.current_buffer,
                    sprite_sheet,
                    sprite,
                    current_palette,
                    scale,
                );

                // Second Image
                add_image(
                    ui,
                    "Preview:",
                    &mut self.preview_buffer,
                    sprite_sheet,
                    sprite,
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
    buffer: &mut Vec<u8>,
    sheet: &SpriteSheet,
    sprite: &[ColorIndex],
    palette: &Palette,
    scale: usize,
) {
    ui.label(label);

    buffer.clear();

    sprite.iter().for_each(|color_index| {
        let rgba = palette[*color_index].into_pixel_data();
        buffer.extend(rgba);
    });

    let image = ColorImage::from_rgba_unmultiplied([sheet.width, sheet.height], &buffer);

    let image = ui.ctx().load_texture(label, image);

    ui.add(Image::new(
        &image,
        Vec2 {
            x: (sheet.width * scale) as f32,
            y: (sheet.height * scale) as f32,
        },
    ));
}
