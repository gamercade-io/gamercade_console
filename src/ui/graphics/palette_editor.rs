use eframe::egui::Ui;

#[derive(Debug, Clone)]
pub struct PaletteEditor {}

impl Default for PaletteEditor {
    fn default() -> Self {
        Self {}
    }
}

impl PaletteEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.columns(2, |columns| {
            self.draw_palette_list(&mut columns[0]);
            self.draw_right_side(&mut columns[1]);
        });
    }

    // Draws the left side panel which displays the palette list widget
    // and related buttons
    fn draw_palette_list(&mut self, ui: &mut Ui) {
        ui.label("Palette List");
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(&mut self, ui: &mut Ui) {
        ui.label("Palette Viewer");

        //TODO: Draw the palette viewer
        ui.columns(2, |columns| {
            self.draw_color_editor(&mut columns[0]);
            self.draw_sprite_preview(&mut columns[1]);
        });
    }

    // Draws the color editor widget
    fn draw_color_editor(&mut self, ui: &mut Ui) {
        ui.label("Color Editor");
    }

    fn draw_sprite_preview(&mut self, ui: &mut Ui) {
        ui.label("Sprite Preview");
    }
}
