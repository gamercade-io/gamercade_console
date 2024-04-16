use gamercade_interface::{game::UpdateGameRequest, release::CreateReleaseRequest};
use rfd::FileDialog;

use crate::{
    app::AppDrawContext,
    task_manager::{GameRequest, UploadReleaseRequest},
    urls::WithSession,
};

#[derive(Default)]
pub struct CreatorDashboardView {
    game_id: String,
    release_name: String,
}

impl CreatorDashboardView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        let enabled = context.auth_state.get_session().is_some();

        ui.set_enabled(enabled);

        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {
            context
                .task_manager
                .game
                .send(GameRequest::CreateGame(WithSession {
                    session: context.auth_state.get_session().unwrap(),
                    data: UpdateGameRequest {
                        game_id: None,
                        title: Some("Test Game".to_string()),
                        short_description: Some("A game for testing".to_string()),
                        long_description: Some("Some more details....".to_string()),
                    },
                }))
        }

        ui.separator();

        ui.label("Game Id:");
        ui.text_edit_singleline(&mut self.game_id);

        ui.label("Release Name or ID:");
        ui.text_edit_singleline(&mut self.release_name);

        if ui.button("Manage Game").clicked() {
            println!("TODO!");
        }

        let game_id = self.game_id.parse();

        if ui.button("Create Release").clicked() {
            if let Ok(game_id) = game_id {
                context.task_manager.release.try_create_release(
                    CreateReleaseRequest {
                        game_id,
                        release_name: self.release_name.clone(),
                    },
                    &context.auth_state.get_session().unwrap(),
                )
            } else {
                self.game_id = String::new()
            }
        }

        if ui.button("Upload Release").clicked() {
            if let (Ok(game_id), Ok(release_id)) = (game_id, self.release_name.parse()) {
                if let Some(file) = FileDialog::new()
                    .add_filter("gcrom (.gcrom)", &["gcrom"])
                    .pick_file()
                {
                    let bytes = std::fs::read(file).unwrap();

                    context.task_manager.release.try_upload_release(
                        UploadReleaseRequest {
                            game_id,
                            release_id,
                            bytes,
                        },
                        &context.auth_state.get_session().unwrap(),
                    )
                }
            }
        }
    }
}
