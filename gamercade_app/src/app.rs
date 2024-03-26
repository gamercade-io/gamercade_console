use std::sync::Arc;

use eframe::egui::{self};
use tokio::sync::Mutex;

use crate::{
    auth::AuthClient,
    local_directory::LocalDirectory,
    task_manager::{TagManager, TagManagerState, TagRequest, TaskManager, TaskRequest},
    view::ActiveView,
};

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    active_view: ActiveView,

    directory: LocalDirectory,

    tag_manager: TagManager,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Fetch Tags").clicked() {
                self.tag_manager.send_request(TagRequest::Initialize)
            }

            self.active_view
                .draw(ui, &mut self.auth_client, &mut self.directory);
        });
    }
}
