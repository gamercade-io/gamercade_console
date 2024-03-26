use eframe::egui::Ui;

use crate::local_directory::LocalDirectory;

use super::ActiveView;

#[derive(Default)]
pub struct OnlineBrowsingView {}

impl OnlineBrowsingView {
    pub fn draw(&mut self, ui: &mut Ui, directory: &mut LocalDirectory) -> Option<ActiveView> {
        ui.label("Online Browsing");

        //TODO: Draw the browsing games list
        directory.iter_games().for_each(|_game| {});

        //TODO: Add a back button for signing in again

        if ui.button("Back").clicked() {
            Some(ActiveView::login())
        } else {
            None
        }
    }
}
