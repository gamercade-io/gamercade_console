// Own imports
mod color_editor;
mod palette_list;
mod palette_viewer;
mod sprite_preview;

use color_editor::ColorEditor;
use palette_list::PaletteList;
use palette_viewer::PaletteViewer;
use sprite_preview::SpritePreview;

// Externals
use eframe::{
    egui::{TextureHandle, Ui},
    epaint::{ColorImage, TextureId},
};

use super::SpriteSheetEditor;

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
    pub fn draw(&mut self, ui: &mut Ui, sprite_sheet_editor: &SpriteSheetEditor) {
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
            self.draw_right_side(ui, texture_id, sprite_sheet_editor)
        });
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(
        &mut self,
        ui: &mut Ui,
        texture_id: TextureId,
        sprite_sheet_editor: &SpriteSheetEditor,
    ) {
        ui.vertical(|ui| {
            let palette = self.palette_list.get_palette_mut();
            self.palette_viewer.draw(ui, palette, texture_id);

            ui.horizontal(|ui| {
                let color = self.palette_viewer.get_color_mut(palette);
                self.color_editor.draw(ui, color, texture_id);

                let color_index = self.palette_viewer.selected_color;
                let preview_color = self.color_editor.preview;
                self.sprite_preview.draw(
                    ui,
                    palette,
                    color_index,
                    preview_color,
                    sprite_sheet_editor,
                );
            });
        });
    }

    pub fn len(&self) -> u8 {
        self.palette_list.len()
    }
}
