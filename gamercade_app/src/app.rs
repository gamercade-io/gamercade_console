use eframe::egui::{self, Ui};
use gamercade_interface::{platform::FrontPageRequest, CRC};

use crate::{
    local_directory::LocalDirectory,
    modes::{AppMode, ArcadeModeView, LibraryModeView, SettingsModeView},
    task_manager::{
        AuthState, GameResponse, HttpResponse, PlatformRequest, PlatformResponse, SuperTaskManager,
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

                    match &self.auth_state {
                        AuthState::Unauthorized => self.modes.arcade.logged_out(),
                        AuthState::SessionHeld(session) => {
                            self.modes.arcade.logged_in();
                            self.tasks
                                .platform
                                .send(PlatformRequest::FrontPage(FrontPageRequest {}));
                            self.tasks
                                .platform
                                .send(PlatformRequest::VotedGames(session.clone()));
                            self.tasks
                                .platform
                                .send(PlatformRequest::EditableGames(session.clone()));
                        }
                    }
                }
                TaskNotification::LoginFailed => self.modes.arcade.logged_out(),
                TaskNotification::PlatformResponse(response) => {
                    self.handle_platform_response(response)
                }
                TaskNotification::GameResponse(response) => self.handle_game_response(response),
                TaskNotification::HttpResponse(response) => self.handle_http_response(response),
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

                self.modes.arcade.online.dashboard.main_view();

                match result {
                    Ok(game_info) => update = Some(game_info),
                    Err(e) => println!("Create Game Error: {e}"),
                }
            }
            GameResponse::UpdateGame(result) => {
                self.modes
                    .arcade
                    .online
                    .dashboard
                    .new_game_view
                    .awaiting_game = false;

                self.modes.arcade.online.dashboard.main_view();

                match result {
                    Ok(game_info) => update = Some(game_info),
                    Err(e) => println!("Update Game Error: {e}"),
                }
            }
        }

        if let Some(game_info) = update {
            self.directory.update_game(game_info);
            self.tasks.platform.send(PlatformRequest::EditableGames(
                self.auth_state.get_session().unwrap(),
            ))
        }
    }

    fn handle_platform_response(&mut self, response: PlatformResponse) {
        match response {
            PlatformResponse::FrontPage(mut front_page_response) => {
                self.modes.arcade.online.arcade.front_page = Some(front_page_response.clone());
                front_page_response
                    .games
                    .drain(..)
                    .for_each(|game| self.directory.update_game(game))
            }

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

    fn handle_http_response(&mut self, response: HttpResponse) {
        match response {
            HttpResponse::DownloadComplete(complete) => {
                let checksum = CRC.checksum(&complete.data);
                let len = complete.data.len();
                self.directory
                    .update_game_rom(complete.game_id, checksum as i64, len as i32);
            }
            HttpResponse::Upload(result) => {
                self.modes
                    .arcade
                    .online
                    .dashboard
                    .manage_game_view
                    .awaiting_upload = false;

                self.modes.arcade.online.dashboard.main_view();

                // TODO: Could add the game to the local directory

                if let Err(e) = result {
                    println!("Upload error {e}")
                }
            }
        }
    }
}
