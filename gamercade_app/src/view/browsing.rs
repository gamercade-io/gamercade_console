use eframe::egui::Ui;

use super::ActiveView;

#[derive(Default)]
pub struct BrowsingView {}

impl BrowsingView {
    pub fn draw(&mut self, ui: &mut Ui) -> Option<ActiveView> {
        ui.label("Browsing");

        None
    }
}
