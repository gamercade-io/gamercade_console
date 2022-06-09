// Own imports
mod color_editor;
mod palette_list;
mod palette_viewer;
mod sprite_preview;

use color_editor::ColorEditor;
use gamercade_core::{Palette, SpriteIndex, SpriteSheet};
use palette_list::PaletteList;
use palette_viewer::PaletteViewer;
use sprite_preview::SpritePreview;

// Externals
use egui::{TextureId, Ui};

use super::SpriteSheetEditor;
use crate::editor_data::EditorGraphicsData;

#[derive(Clone, Default)]
pub struct PaletteEditor {
    palette_list: PaletteList,
    palette_viewer: PaletteViewer,
    color_editor: ColorEditor,
    sprite_preview: SpritePreview,
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
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorGraphicsData,
        sprite_sheet_editor: &SpriteSheetEditor,
        scale: usize,
        texture_id: TextureId,
    ) {
        ui.horizontal(|ui| {
            self.palette_list.draw(ui, texture_id, data);

            let palette = &mut data.palettes[self.palette_list.selected_palette].palette;

            let sheet = sprite_sheet_editor.selected_sheet();
            let sprite = sprite_sheet_editor.selected_sprite();
            self.draw_right_side(
                ui,
                texture_id,
                palette,
                &data.sprite_sheets[sheet.0 as usize].sprite_sheet,
                sprite,
                scale,
            )
        });
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(
        &mut self,
        ui: &mut Ui,
        texture_id: TextureId,
        palette: &mut Palette,
        sprite_sheet: &SpriteSheet,
        sprite_index: SpriteIndex,
        scale: usize,
    ) {
        ui.vertical(|ui| {
            self.palette_viewer.draw(ui, palette, texture_id);

            ui.horizontal(|ui| {
                let color = self.palette_viewer.get_color_mut(palette);
                self.color_editor.draw(ui, color, texture_id);

                let mut preview_palette = palette.clone();
                preview_palette.colors[self.palette_viewer.selected_color] =
                    self.color_editor.preview;

                self.sprite_preview.draw(
                    ui,
                    palette,
                    &preview_palette,
                    sprite_sheet,
                    sprite_index,
                    scale,
                );
            });
        });
    }

    pub fn selected_palette_mut(&mut self) -> &mut usize {
        &mut self.palette_list.selected_palette
    }
}
