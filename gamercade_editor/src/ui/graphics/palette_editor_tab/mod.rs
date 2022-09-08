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
use eframe::egui::{SidePanel, TextureId, TopBottomPanel, Ui};

use super::SpriteSheetEditor;
use crate::editor_data::{EditorGraphicsData, EditorPalette};

#[derive(Clone, Default)]
pub struct PaletteEditor {
    palette_list: PaletteList,
    palette_viewer: PaletteViewer,
    color_editor: ColorEditor,
    sprite_preview: SpritePreview,
}

impl PaletteEditor {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorGraphicsData,
        sprite_sheet_editor: &SpriteSheetEditor,
        scale: f32,
        texture_id: TextureId,
    ) {
        // Draw Palette List
        SidePanel::left("palette_list_left_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                TopBottomPanel::bottom("palette_list_bottom_panel")
                    .show_inside(ui, |ui| self.palette_list.draw_buttons(ui, data));
                self.palette_list.draw(ui, texture_id, data);
            });

        // Draw Sprite Preview
        let palette = &mut data.palettes[self.palette_list.selected_palette];

        let sheet = sprite_sheet_editor.selected_sheet();
        let sprite_index = sprite_sheet_editor.selected_sprite();
        let sprite_sheet = &data.sprite_sheets[sheet.0 as usize].sprite_sheet;

        let mut preview_palette = palette.palette.clone();
        preview_palette.colors[self.palette_viewer.selected_color] = self.color_editor.preview;

        SidePanel::right("sprite_preview_right_panel")
            .resizable(false)
            .width_range(640.0..=f32::INFINITY)
            .show_inside(ui, |ui| {
                self.sprite_preview.draw(
                    ui,
                    &palette.palette,
                    &preview_palette,
                    sprite_sheet,
                    sprite_index,
                    scale,
                );
            });

        self.draw_color_editor(ui, texture_id, palette)
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_color_editor(
        &mut self,
        ui: &mut Ui,
        texture_id: TextureId,
        palette: &mut EditorPalette,
    ) {
        ui.vertical(|ui| {
            self.palette_viewer.draw(ui, palette, texture_id);

            ui.horizontal(|ui| {
                let color = self.palette_viewer.get_color_mut(&mut palette.palette);
                self.color_editor.draw(ui, color, texture_id);
            });
        });
    }

    pub fn selected_palette_mut(&mut self) -> &mut usize {
        &mut self.palette_list.selected_palette
    }
}
