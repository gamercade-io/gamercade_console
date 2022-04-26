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
        ui.horizontal(|ui| {
            self.draw_palette_list(ui);
            self.draw_right_side(ui)
        });
    }

    // Draws the left side panel which displays the palette list widget
    // and related buttons
    fn draw_palette_list(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Palette List");
        });
    }

    // Draws the right side panel which includes palette viewer, color
    // editor, and sprite preview widgets
    fn draw_right_side(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Palette Viewer");

                ui.horizontal(|ui| {
                    self.draw_color_editor(ui);
                    self.draw_sprite_preview(ui);
                });
            });
        });
    }

    // Draws the color editor widget
    fn draw_color_editor(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Color Editor");
            });
        });
    }

    fn draw_sprite_preview(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview");
            });
        });
    }
}
