use egui::{Color32, Image, TextureId, Ui, Vec2};
use gamercade_core::{ColorIndex, PALETTE_COLORS};

use crate::editor_data::EditorPalette;

#[derive(Debug, Clone, Default)]
pub struct PalettePreview {}

impl PalettePreview {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        palettes: &[EditorPalette],
        selected_palette: &mut usize,
        texture_id: TextureId,
    ) {
        ui.group(|ui| {
            ui.label("Palette Previewer");

            ui.horizontal(|ui| {
                palettes.iter().enumerate().for_each(|(index, palette)| {
                    draw_preview(ui, index, palette, selected_palette, texture_id)
                });
            })
        });
    }
}

fn draw_preview(
    ui: &mut Ui,
    index: usize,
    palette: &EditorPalette,
    selected_palette: &mut usize,
    texture_id: TextureId,
) {
    // TODO: Make this whole thing clickable
    ui.vertical(|ui| {
        ui.selectable_value(selected_palette, index, &palette.name);

        ui.horizontal(|ui| {
            (0..PALETTE_COLORS / 2).for_each(|i| {
                let i = i as u8;
                ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };
                ui.vertical(|ui| {
                    let color_top = palette.palette[ColorIndex(i)];
                    let color_bottom = palette.palette[ColorIndex(i + (PALETTE_COLORS as u8 / 2))];

                    let image_top = Image::new(texture_id, Vec2 { x: 10.0, y: 10.0 })
                        .tint(Color32::from_rgb(color_top.r, color_top.g, color_top.b));

                    let image_bottom = Image::new(texture_id, Vec2 { x: 10.0, y: 10.0 }).tint(
                        Color32::from_rgb(color_bottom.r, color_bottom.g, color_bottom.b),
                    );

                    ui.add(image_top);
                    ui.add(image_bottom);
                });
            })
        })
    });
}
