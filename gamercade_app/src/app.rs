use eframe::egui::{self};

use crate::{
    auth::AuthClient,
    local_directory::LocalDirectory,
    task_manager::{SuperTaskManager, TagRequest, TaskNotification},
    view::ActiveView,
};

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    active_view: ActiveView,

    directory: LocalDirectory,

    tasks: SuperTaskManager,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_notifications();

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Remove this and fetch tags / Author Levels automatically
            if ui.button("Fetch Tags").clicked() {
                self.tasks.tags.send_request(TagRequest::Initialize)
            }

            self.active_view
                .draw(ui, &mut self.auth_client, &mut self.directory);
        });
    }
}

impl App {
    fn handle_notifications(&mut self) {
        while let Ok(notification) = self.tasks.events.try_recv() {
            match notification {
                TaskNotification::GlobalTags(tags) => {
                    self.directory.upsert_tags(&tags, true);
                }
                TaskNotification::GlobalPermissionLevels(permissions) => {
                    self.directory.upsert_permission_levesl(&permissions, true);
                }
            }
        }
    }
}
