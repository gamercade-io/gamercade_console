use eframe::egui::Ui;

#[derive(Debug, Clone, Default)]
pub struct SheetEditor {
    pub selected_sprite: usize,
}

impl SheetEditor {
    pub fn draw(&mut self, ui: &mut Ui) {
        //TODO
        ui.group(|ui| {
            ui.label("Sprite Sheet Editor");
        });
    }
}
