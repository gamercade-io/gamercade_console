// Own imports
mod palette_preview;
mod sheet_editor;
mod sheet_list;
mod sheet_settings;

use gamercade_core::{SpriteIndex, SpriteSheetIndex};
use palette_preview::PalettePreview;
use sheet_editor::SheetEditor;
use sheet_list::SheetList;
use sheet_settings::SheetSettings;

// Externals
use egui::Ui;

use super::PaletteEditor;
use crate::editor_data::EditorGraphicsData;

#[derive(Debug, Clone, Default)]
pub struct SpriteSheetEditor {
    list: SheetList,
    settings: SheetSettings,
    editor: SheetEditor,
    palette_preview: PalettePreview,
}

impl SpriteSheetEditor {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorGraphicsData,
        palette_editor: &PaletteEditor,
        scale: usize,
    ) {
        ui.horizontal(|ui| {
            self.list.draw(ui, data);

            let palette_len = data.palettes.len() as u8;
            let sheet = &mut data.sprite_sheets[self.list.selected_sheet.0 as usize];
            let palettes = &mut data.palettes;
            let selected_palette = palette_editor.selected_palette();

            ui.vertical(|ui| {
                self.settings.draw(ui, sheet, palette_len);

                self.editor.draw(
                    ui,
                    &mut sheet.sprite_sheet,
                    scale,
                    palettes,
                    selected_palette,
                );
                self.palette_preview.draw(ui);
            });
        });
    }

    pub fn selected_sheet(&self) -> SpriteSheetIndex {
        self.list.selected_sheet
    }

    pub fn selected_sprite(&self) -> SpriteIndex {
        self.editor.selected_sprite
    }
}
