// Own imports
mod palette_preview;
mod sheet_editor;
mod sheet_list;
mod sheet_settings;

use gamercade_core::{ColorIndex, Palette, SpriteIndex, SpriteSheetIndex};
use hashbrown::HashMap;
use palette_preview::PalettePreview;
use sheet_editor::SheetEditor;
use sheet_list::SheetList;
use sheet_settings::SheetSettings;

// Externals
use egui::{TextureId, Ui};

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
        palette_editor: &mut PaletteEditor,
        scale: usize,
        texture_id: TextureId,
    ) {
        ui.horizontal(|ui| {
            self.list.draw(ui, data);

            let palette_len = data.palettes.len() as u8;
            let sheet = &mut data.sprite_sheets[self.list.selected_sheet.0 as usize];
            let selected_palette = palette_editor.selected_palette_mut();
            let palette = &data.palettes[*selected_palette].palette;

            ui.vertical(|ui| {
                self.settings.draw(ui, sheet, palette_len);

                self.editor
                    .draw(ui, &mut sheet.sprite_sheet, scale, palette);
                self.palette_preview
                    .draw(ui, &data.palettes, selected_palette, texture_id);
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

pub(crate) fn palette_to_map(palette: &Palette) -> HashMap<image::Rgb<u8>, ColorIndex> {
    palette
        .colors
        .iter()
        .enumerate()
        .map(|(index, color)| {
            (
                image::Rgb::<u8>([color.r, color.g, color.b]),
                ColorIndex(index as u8),
            )
        })
        .collect()
}
