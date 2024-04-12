use crate::app::AppDrawContext;

use super::ArcadeActiveView;

#[derive(Default)]
pub struct OnlineBrowsingView {
    release_id: String,
}

impl OnlineBrowsingView {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ArcadeActiveView> {
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
            Some(ArcadeActiveView::login())
        } else {
            None
        }
    }
}
