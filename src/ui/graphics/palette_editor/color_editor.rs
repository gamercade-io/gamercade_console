use eframe::egui::Ui;

#[derive(Clone, Debug, Default)]
pub struct ColorEditor {}

impl ColorEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        //TODO:
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Color Editor");
            });
        });
    }
}
