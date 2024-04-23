use eframe::egui::{TextureHandle, Ui};

use crate::ui::draw_palette_preview;
use gamercade_fs::EditorPalette;

#[derive(Debug, Clone, Default)]
pub struct PalettePreview {}

impl PalettePreview {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        palettes: &[EditorPalette],
        selected_palette: &mut usize,
        texture_handle: &TextureHandle,
    ) {
        ui.group(|ui| {
            ui.label("Palette Previewer");

            ui.horizontal(|ui| {
                palettes.iter().enumerate().for_each(|(index, palette)| {
                    draw_preview(ui, index, palette, selected_palette, texture_handle)
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
    texture_handle: &TextureHandle,
) {
    // TODO: Make this whole thing clickable
    ui.vertical(|ui| {
        ui.selectable_value(selected_palette, index, &palette.name);

        draw_palette_preview(ui, &palette.palette, texture_handle);
    });
}
