use eframe::egui::Ui;

use crate::{app::AppDrawContext, local_directory::LocalDirectory, task_manager::SuperTaskManager};

use super::ActiveView;

#[derive(Default)]
pub struct OnlineBrowsingView {
    release_id: String,
}

impl OnlineBrowsingView {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ActiveView> {
        let AppDrawContext { ui, directory, .. } = context;

        ui.label("Online Browsing");

        //TODO: Draw the browsing games list
        //TODO: Add search parameters etc

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
