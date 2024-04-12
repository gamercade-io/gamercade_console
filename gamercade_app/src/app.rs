use eframe::egui::{self, Ui};

use crate::{
    local_directory::LocalDirectory,
    modes::{AppMode, ArcadeActiveView, ArcadeModeView, LibraryModeView, SettingsModeView},
    task_manager::{AuthState, SuperTaskManager, TaskNotification},
};

#[derive(Default)]
pub struct App {
    directory: LocalDirectory,

    tasks: SuperTaskManager,
    auth_state: AuthState,

    active_mode: AppMode,
    modes: Modes,
}

#[derive(Default)]
pub struct Modes {
    arcade: ArcadeModeView,
    library: LibraryModeView,
    settings: SettingsModeView,
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

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_mode, AppMode::Arcade, "Arcade");
                ui.selectable_value(&mut self.active_mode, AppMode::Library, "Library");
                ui.selectable_value(&mut self.active_mode, AppMode::Settings, "Settings");
            });

            ui.separator();

            let context = AppDrawContext {
                ui,
                task_manager: &mut self.tasks,
                directory: &mut self.directory,
            };

            match self.active_mode {
                AppMode::Arcade => self.modes.arcade.draw(context),
                AppMode::Library => self.modes.library.draw(context),
                AppMode::Settings => self.modes.settings.draw(context),
            }
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
                        AuthState::Unauthorized => self.modes.arcade.logged_in(),
                        AuthState::SessionHeld(_) => self.modes.arcade.logged_out(),
                    }
                }
            }
        }
    }
}
