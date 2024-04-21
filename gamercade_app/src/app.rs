use eframe::egui::{self, Ui};
use gamercade_interface::{platform::FrontPageRequest, CRC};

use crate::{
    local_directory::LocalDirectory,
    modes::{AppMode, ArcadeModeView, LibraryModeView, SettingsModeView},
    task_manager::{
        AuthState, GameResponse, PlatformRequest, PlatformResponse, SuperTaskManager,
        TaskNotification,
    },
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
    pub arcade: ArcadeModeView,
    pub library: LibraryModeView,
    pub settings: SettingsModeView,
}

pub struct AppDrawContext<'a> {
    pub ui: &'a mut Ui,
    pub task_manager: &'a mut SuperTaskManager,
    pub directory: &'a mut LocalDirectory,
    pub auth_state: &'a AuthState,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_notifications();

        self.directory.sync_games_cache();

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Front Page").clicked() {
                self.tasks
                    .platform
                    .send(crate::task_manager::PlatformRequest::FrontPage(
                        FrontPageRequest {},
                    ))
            }

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_mode, AppMode::Arcade, "Arcade");
                ui.selectable_value(&mut self.active_mode, AppMode::Library, "Library");
                ui.selectable_value(&mut self.active_mode, AppMode::Settings, "Settings");
            });

            ui.separator();

            let context = &mut AppDrawContext {
                ui,
                task_manager: &mut self.tasks,
                directory: &mut self.directory,
                auth_state: &self.auth_state,
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
                    self.auth_state = new_state;

                    match self.auth_state {
                        AuthState::Unauthorized => self.modes.arcade.logged_out(),
                        AuthState::SessionHeld(_) => {
                            self.modes.arcade.logged_in();
                            self.tasks.platform.send(PlatformRequest::VotedGames);
                            self.tasks.platform.send(PlatformRequest::EditableGames);
                        }
                    }
                }
                TaskNotification::LoginFailed => self.modes.arcade.logged_out(),
                TaskNotification::DownloadRomComplete(complete) => {
                    let checksum = CRC.checksum(&complete.data);
                    let len = complete.data.len();
                    self.directory
                        .update_game_rom(complete.game_id, checksum as i64, len as i32);
                }
                TaskNotification::PlatformResponse(response) => {
                    self.handle_platform_response(response)
                }
                TaskNotification::GameResponse(response) => self.handle_game_response(response),
            }
        }
    }

    fn handle_game_response(&mut self, response: GameResponse) {
        let mut update = None;
        match response {
            GameResponse::CreateGame(result) => {
                self.modes
                    .arcade
                    .online
                    .dashboard
                    .new_game_view
                    .awaiting_game = false;

                match result {
                    Ok(game_info) => update = Some(game_info),
                    Err(e) => println!("Create Game Error: {e}"),
                }
            }
            GameResponse::UpdateGame(result) => {
                // TODO: Something here?
                match result {
                    Ok(game_info) => update = Some(game_info),
                    Err(e) => println!("Update Game Error: {e}"),
                }
            }
        }

        if let Some(game_info) = update {
            self.directory.update_game(game_info)
        }
    }

    fn handle_platform_response(&mut self, response: PlatformResponse) {
        match response {
            PlatformResponse::FrontPage(mut front_page_response) => front_page_response
                .games
                .drain(..)
                .for_each(|game| self.directory.update_game(game)),

            PlatformResponse::EditableGames(editable_games_response) => self
                .directory
                .handle_editable_games_response(editable_games_response),

            PlatformResponse::VotedGames(voted_games_response) => self
                .directory
                .handle_voted_games_response(voted_games_response),

            PlatformResponse::Search(mut search_response) => search_response
                .games_info
                .drain(..)
                .for_each(|game| self.directory.update_game(game)),
        }
    }
}
