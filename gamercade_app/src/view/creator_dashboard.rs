use crate::app::AppDrawContext;

use super::ActiveView;

pub struct CreatorDashboard {}

impl CreatorDashboard {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ActiveView> {
        let ui = context.ui;

        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {}

        if ui.button("Manage Game").clicked() {}

        if ui.button("Create Release").clicked() {}

        if ui.button("Back").clicked() {
            return Some(ActiveView::online_browsing());
        }

        None
    }
}
