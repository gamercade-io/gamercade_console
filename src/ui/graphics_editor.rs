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
    pub fn draw(&mut self, ui: &mut Ui) {
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

        match self.mode {
            GraphicsEditorMode::PaletteEditor => self.palette_editor(ui),
            GraphicsEditorMode::SpriteSheetEditor => self.sprite_sheet_editor(ui),
            GraphicsEditorMode::SpriteEditor => self.sprite_editor(ui),
        };
    }

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
