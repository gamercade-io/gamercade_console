use eframe::egui::Ui;

#[derive(Debug, Clone)]
pub struct SpriteSheetEditor {}

impl Default for SpriteSheetEditor {
    fn default() -> Self {
        Self {}
    }
}

impl SpriteSheetEditor {
    pub fn draw(&mut self, ui: &mut Ui) {}
}
