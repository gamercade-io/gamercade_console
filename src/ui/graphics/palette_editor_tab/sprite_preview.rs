use egui::{ColorImage, Image, TextureHandle, Ui, Vec2};
use gamercade_core::{ColorIndex, Palette, SpriteIndex, SpriteSheet};

use crate::ui::load_buffered_image;

#[derive(Clone, Default)]
pub struct SpritePreview {
    current: SpritePreviewEntry,
    preview: SpritePreviewEntry,
}

#[derive(Clone, Default)]
struct SpritePreviewEntry {
    label: &'static str,
    rgb_buffer: Vec<u8>,
    texture_handle: Option<TextureHandle>,
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
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview: ");

                let sprite = &sprite_sheet[sprite_index];

                // First Image
                add_image(
                    ui,
                    &mut self.current,
                    sprite_sheet,
                    sprite,
                    current_palette,
                    scale,
                );

                // Second Image
                add_image(
                    ui,
                    &mut self.preview,
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
    entry: &mut SpritePreviewEntry,
    sheet: &SpriteSheet,
    sprite: &[ColorIndex],
    palette: &Palette,
    scale: usize,
) {
    ui.label(entry.label);
    entry.rgb_buffer.clear();

    sprite.iter().for_each(|color_index| {
        let rgba = palette[*color_index].into_pixel_data();
        entry.rgb_buffer.extend(rgba);
    });

    let rgb = ColorImage::from_rgba_unmultiplied([sheet.width, sheet.height], &entry.rgb_buffer);

    let image = load_buffered_image(ui, &mut entry.texture_handle, entry.label, rgb);

    ui.add(Image::new(
        image,
        Vec2 {
            x: (sheet.width * scale) as f32,
            y: (sheet.height * scale) as f32,
        },
    ));
}
