use egui::Ui;

#[derive(Debug, Clone, Default)]
pub struct PalettePreview {}

impl PalettePreview {
    pub fn draw(&mut self, ui: &mut Ui) {
        //TODO
        ui.group(|ui| ui.label("Palette Previewer"));
    }
}
