use crate::app::AppDrawContext;

use super::ArcadeActiveView;

pub struct CreatorDashboardView {}

impl CreatorDashboardView {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ArcadeActiveView> {
        let ui = context.ui;

        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {}

        if ui.button("Manage Game").clicked() {}

        if ui.button("Create Release").clicked() {}

        if ui.button("Back").clicked() {
            return Some(ArcadeActiveView::online_browsing());
        }

        None
    }
}
