use crate::app::AppDrawContext;

#[derive(Default)]
pub struct CreatorDashboardView {}

impl CreatorDashboardView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {}

        if ui.button("Manage Game").clicked() {}

        if ui.button("Create Release").clicked() {}
    }
}
