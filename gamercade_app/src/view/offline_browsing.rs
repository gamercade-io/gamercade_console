use eframe::egui::Ui;

use crate::{app::AppDrawContext, local_directory::LocalDirectory};

use super::ActiveView;

// TODO: Consider adding a new tab like "Games Library"
// Or something relevant

#[derive(Default)]
pub struct OfflineBrowsingView {}

impl OfflineBrowsingView {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ActiveView> {
        let AppDrawContext { ui, directory, .. } = context;
        ui.label("Offline Browsing");

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
