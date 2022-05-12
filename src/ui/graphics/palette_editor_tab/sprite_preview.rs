use eframe::egui::Ui;
use gamercade_core::{Color, Palette};

use crate::ui::SpriteSheetEditor;

#[derive(Clone, Debug, Default)]
pub struct SpritePreview {}

impl SpritePreview {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        _palette: &Palette,
        _preview_index: usize,
        _preview_color: Color,
        _sprite_sheet_editor: &SpriteSheetEditor,
    ) {
        // TODO: Write this!
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview");
            });
        });
    }
}
