mod color_editor;
mod palette_list;
mod palette_viewer;
mod sprite_preview;

use eframe::egui::Ui;

use self::color_editor::ColorEditor;
use self::palette_list::PaletteList;
use self::palette_viewer::PaletteViewer;
use self::sprite_preview::SpritePreview;

#[derive(Debug, Clone, Default)]
pub struct PaletteEditor {
    palette_list: PaletteList,
    palette_viewer: PaletteViewer,
    color_editor: ColorEditor,
    sprite_preview: SpritePreview,
}

impl PaletteEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.palette_list.draw(ui);
            self.draw_right_side(ui)
        });
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let palette = self.palette_list.get_palette();
            self.palette_viewer.draw(ui, palette);

            ui.horizontal(|ui| {
                self.color_editor.draw(ui);
                self.sprite_preview.draw(ui);
            });
        });
    }
}
