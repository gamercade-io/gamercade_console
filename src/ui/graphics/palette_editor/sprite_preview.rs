use eframe::egui::Ui;

#[derive(Clone, Debug, Default)]
pub struct SpritePreview {}

impl SpritePreview {
    pub fn draw(&mut self, ui: &mut Ui) {
        //TODO:
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Sprite Preview");
            });
        });
    }
}
