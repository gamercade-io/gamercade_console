use eframe::egui::{self, Ui};

use crate::{
    local_directory::LocalDirectory,
    task_manager::{AuthState, SuperTaskManager, TagRequest, TaskNotification},
    view::ActiveView,
};

#[derive(Default)]
pub struct App {
    active_view: ActiveView,

    directory: LocalDirectory,

    tasks: SuperTaskManager,
    auth_state: AuthState,
}

pub struct AppDrawContext<'a> {
    pub ui: &'a mut Ui,
    pub task_manager: &'a mut SuperTaskManager,
    pub directory: &'a mut LocalDirectory,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_notifications();

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Remove this and fetch tags / Author Levels automatically
            // if ui.button("Fetch Tags").clicked() {
            //     self.tasks.tags.send_request(TagRequest::Initialize)
            // }

            let context = AppDrawContext {
                ui,
                task_manager: &mut self.tasks,
                directory: &mut self.directory,
            };

            self.active_view.draw(context);
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
                TaskNotification::AuthStateChanged(new_state) => {
                    println!("Auth State Changed: {new_state:?}");
                    self.auth_state = new_state;

                    match self.auth_state {
                        AuthState::Unauthorized => self.active_view = ActiveView::login(),
                        AuthState::SessionHeld(_) => {
                            self.active_view = ActiveView::online_browsing()
                        }
                    }
                }
            }
        }
    }
}
