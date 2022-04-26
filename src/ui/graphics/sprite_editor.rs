use eframe::egui::Ui;

#[derive(Debug, Clone)]
pub struct SpriteEditor {}

impl Default for SpriteEditor {
    fn default() -> Self {
        Self {}
    }
}

impl SpriteEditor {
    pub fn draw(&mut self, ui: &mut Ui) {}
}
