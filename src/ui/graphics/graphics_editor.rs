use eframe::egui::Ui;

use super::{PaletteEditor, SpriteEditor, SpriteSheetEditor};

#[derive(Clone, Debug, PartialEq)]
pub enum GraphicsEditorMode {
    PaletteEditor,
    SpriteSheetEditor,
    SpriteEditor,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::PaletteEditor,
            palette_editor: PaletteEditor::default(),
            sprite_sheet_editor: SpriteSheetEditor::default(),
            sprite_editor: SpriteEditor::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphicsEditor {
    pub mode: GraphicsEditorMode,
    pub palette_editor: PaletteEditor,
    pub sprite_sheet_editor: SpriteSheetEditor,
    pub sprite_editor: SpriteEditor,
}

impl GraphicsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(
            &mut self.mode,
            GraphicsEditorMode::PaletteEditor,
            "Palettes",
        );
        ui.selectable_value(
            &mut self.mode,
            GraphicsEditorMode::SpriteSheetEditor,
            "Sprite Sheets",
        );
        ui.selectable_value(
            &mut self.mode,
            GraphicsEditorMode::SpriteEditor,
            "Sprite Editor",
        );
    }

    pub fn draw_contents(&mut self, ui: &mut Ui) {
        match self.mode {
            GraphicsEditorMode::PaletteEditor => self.palette_editor.draw(ui),
            GraphicsEditorMode::SpriteSheetEditor => self.sprite_sheet_editor.draw(ui),
            GraphicsEditorMode::SpriteEditor => self.sprite_editor.draw(ui),
        };
    }
}
