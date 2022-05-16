// Own imports
mod palette_preview;
mod sheet_editor;
mod sheet_list;
mod sheet_settings;

use palette_preview::PalettePreview;
use sheet_editor::SheetEditor;
use sheet_list::SheetList;
use sheet_settings::SheetSettings;

// Externals
use eframe::egui::Ui;

use crate::editor_data::EditorGraphicsData;

#[derive(Debug, Clone, Default)]
pub struct SpriteSheetEditor {
    list: SheetList,
    settings: SheetSettings,
    editor: SheetEditor,
    palette_preview: PalettePreview,
}

impl SpriteSheetEditor {
    pub fn draw(&mut self, ui: &mut Ui, data: &mut EditorGraphicsData) {
        ui.horizontal(|ui| {
            self.list.draw(ui, data);

            let palette_len = data.palettes.len() as u8;
            let sheet = &mut data.sprite_sheets[self.list.selected_sheet];

            ui.vertical(|ui| {
                self.settings.draw(ui, sheet, palette_len);
                self.editor.draw(ui);
                self.palette_preview.draw(ui);
            });
        });
    }
}
