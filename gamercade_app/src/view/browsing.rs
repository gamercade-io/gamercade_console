use eframe::egui::Ui;

use super::ActiveView;

struct LocalGameEntry {
    name: String,
    releases: Vec<LocalGameRelease>,
}

struct LocalGameRelease {}

pub struct BrowsingView {
    local_game_list: Vec<LocalGameEntry>,
}

impl Default for BrowsingView {
    fn default() -> Self {
        //TODO: Populate these fields with local data
        Self {
            local_game_list: Vec::new(),
        }
    }
}

impl BrowsingView {
    pub fn draw(&mut self, ui: &mut Ui) -> Option<ActiveView> {
        ui.label("Browsing");

        //TODO: Draw the browsing games list

        //TODO: Add a back button for signing in again
        None
    }
}
