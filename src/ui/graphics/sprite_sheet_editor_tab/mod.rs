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

use super::PaletteEditor;

#[derive(Debug, Clone, Default)]
pub struct SpriteSheetEditor {
    list: SheetList,
    settings: SheetSettings,
    editor: SheetEditor,
    palette_preview: PalettePreview,
}

impl SpriteSheetEditor {
    pub fn draw(&mut self, ui: &mut Ui, palette_tab: &PaletteEditor) {
        ui.horizontal(|ui| {
            self.list.draw(ui);

            let palette_len = palette_tab.len();
            let sheet = self.list.get_sprite_sheet_mut();

            ui.vertical(|ui| {
                self.settings.draw(ui, sheet, palette_len);
                self.editor.draw(ui);
                self.palette_preview.draw(ui);
            });
        });
    }
}
