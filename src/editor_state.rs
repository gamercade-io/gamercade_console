use eframe::egui::Ui;

pub struct EditorState {
    pub mode: EditorMode,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            mode: EditorMode::PaletteEditor,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorMode {
    PaletteEditor,
    SpriteSheetEditor,
    SpriteEditor,
}

impl EditorState {
    pub fn palette_editor(&mut self, ui: &mut Ui) {
        //TODO!
    }

    pub fn sprite_sheet_editor(&mut self, ui: &mut Ui) {
        //TODO!
    }

    pub fn sprite_editor(&mut self, ui: &mut Ui) {
        //TODO
    }
}
