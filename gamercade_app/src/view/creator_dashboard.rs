use eframe::egui::Ui;

use crate::task_manager::SuperTaskManager;

use super::ActiveView;

pub struct CreatorDashboard {}

impl CreatorDashboard {
    pub fn draw(&mut self, ui: &mut Ui, task_manager: &SuperTaskManager) -> Option<ActiveView> {
        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {}

        if ui.button("Create Release").clicked() {}

        if ui.button("Back").clicked() {
            return Some(ActiveView::online_browsing());
        }

        None
    }
}
