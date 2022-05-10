// Own imports
mod color_editor;
mod palette_list;
mod palette_viewer;
mod sprite_preview;

use self::color_editor::ColorEditor;
use self::palette_list::PaletteList;
use self::palette_viewer::PaletteViewer;
use self::sprite_preview::SpritePreview;

// Externals
use eframe::{
    egui::{TextureHandle, Ui},
    epaint::{ColorImage, TextureId},
};

#[derive(Clone, Default)]
pub struct PaletteEditor {
    palette_list: PaletteList,
    palette_viewer: PaletteViewer,
    color_editor: ColorEditor,
    sprite_preview: SpritePreview,
    default_palette_texture: Option<TextureHandle>,
}

impl std::fmt::Debug for PaletteEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaletteEditor")
            .field("palette_list", &self.palette_list)
            .field("palette_viewer", &self.palette_viewer)
            .field("color_editor", &self.color_editor)
            .field("sprite_preview", &self.sprite_preview)
            .finish()
    }
}

impl PaletteEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        let texture_id = self
            .default_palette_texture
            .get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "default palette texture",
                    ColorImage::from_rgba_unmultiplied([1, 1], &[255, 255, 255, 255]),
                )
            })
            .id();

        ui.horizontal(|ui| {
            self.palette_list.draw(ui, texture_id);
            self.draw_right_side(ui, texture_id)
        });
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(&mut self, ui: &mut Ui, texture_id: TextureId) {
        ui.vertical(|ui| {
            let palette = self.palette_list.get_palette();
            self.palette_viewer.draw(ui, palette, texture_id);

            let color = self.palette_viewer.get_color(palette);

            ui.horizontal(|ui| {
                self.color_editor.draw(ui, color, texture_id);
                self.sprite_preview.draw(ui);
            });
        });
    }
}
