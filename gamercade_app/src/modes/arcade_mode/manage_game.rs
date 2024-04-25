use gamercade_interface::game::UpdateGameRequest;
use rfd::FileDialog;

use crate::{
    app::AppDrawContext,
    local_directory::Game,
    task_manager::{GameRequest, UploadRom},
    urls::WithSession,
};

#[derive(Default)]
pub struct ManageGameView {
    pub awaiting_request: bool,
    game_id: i64,
    title: String,
    short_description: String,
    long_description: String,
    tags: Vec<i32>,
}

impl ManageGameView {
    pub fn new(game: &Game) -> Self {
        Self {
            awaiting_request: false,
            game_id: game.id,
            title: game.title.clone(),
            short_description: game.short_description.clone(),
            long_description: game.long_description.as_ref().cloned().unwrap_or_default(),
            tags: game.tags.clone(),
        }
    }

    // TODO: Add game icon visible here
    // TODO: Add ability to upload game icons
    pub fn draw(&mut self, context: &mut AppDrawContext) -> bool {
        let mut done = false;
        let ui = &mut context.ui;

        ui.set_enabled(!self.awaiting_request);

        ui.label("Manage Game View:");
        ui.separator();

        ui.label("Game Title: ");
        ui.text_edit_singleline(&mut self.title);
        ui.separator();

        ui.label("Short Description: ");
        ui.text_edit_singleline(&mut self.short_description);

        ui.label("Long Description: ");
        ui.text_edit_singleline(&mut self.long_description);
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Upload ROM").clicked() {
                if let Some(file) = FileDialog::new()
                    .add_filter("gcrom (.gcrom)", &["gcrom"])
                    .pick_file()
                {
                    let bytes = std::fs::read(file).unwrap();

                    context.task_manager.http.try_upload_rom(
                        UploadRom {
                            game_id: self.game_id,
                            bytes,
                        },
                        &context.auth_state.get_session().unwrap(),
                    );

                    self.awaiting_request = true;
                }
            }

            if ui.button("Update Game").clicked() {
                context
                    .task_manager
                    .game
                    .send(GameRequest::UpdateGame(WithSession::new(
                        &context.auth_state.get_session().unwrap(),
                        UpdateGameRequest {
                            game_id: Some(self.game_id),
                            title: Some(self.title.clone()),
                            short_description: Some(self.short_description.clone()),
                            long_description: Some(self.long_description.clone()),
                        },
                    )));

                self.awaiting_request = true;
            }
        });

        ui.separator();

        // TODO: Add delete game (via a "are you sure?" confirmation box)
        ui.set_enabled(true);

        if ui.button("Go Back").clicked() {
            done = true;
        }

        if self.awaiting_request {
            ui.spinner();
        }

        done
    }
}
