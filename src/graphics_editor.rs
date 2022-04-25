use eframe::egui::Ui;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphicsEditorMode {
    PaletteEditor,
    SpriteSheetEditor,
    SpriteEditor,
}

impl Default for GraphicsEditor {
    fn default() -> Self {
        Self {
            mode: GraphicsEditorMode::PaletteEditor,
        }
    }
}

pub struct GraphicsEditor {
    pub mode: GraphicsEditorMode,
}

impl GraphicsEditor {
    pub fn palette_editor(&mut self, ui: &mut Ui) {
        //TODO: render the palette editor
    }

    pub fn sprite_sheet_editor(&mut self, ui: &mut Ui) {
        //TODO: render the sprite sheet editor
    }

    pub fn sprite_editor(&mut self, ui: &mut Ui) {
        //TODO: render the sprite editor
    }
}
