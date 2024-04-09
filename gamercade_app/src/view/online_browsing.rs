use eframe::egui::Ui;

use crate::{local_directory::LocalDirectory, task_manager::SuperTaskManager};

use super::ActiveView;

#[derive(Default)]
pub struct OnlineBrowsingView {
    release_id: String,
}

impl OnlineBrowsingView {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        task_manager: &mut SuperTaskManager,
        directory: &mut LocalDirectory,
    ) -> Option<ActiveView> {
        ui.label("Online Browsing");

        //TODO: Draw the browsing games list
        directory.iter_games().for_each(|_game| {});

        ui.horizontal(|ui| {
            ui.label("Release Id: ");
            ui.text_edit_singleline(&mut self.release_id);
            if ui.button("Download Release").clicked() {
                println!("TODO: Download Release")
            }
        });

        if ui.button("Back").clicked() {
            Some(ActiveView::login())
        } else {
            None
        }
    }
}
