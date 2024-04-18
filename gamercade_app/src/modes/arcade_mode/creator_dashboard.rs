use gamercade_interface::game::UpdateGameRequest;
use rfd::FileDialog;

use crate::{
    app::AppDrawContext,
    task_manager::{GameRequest, UploadRom},
    urls::WithSession,
};

#[derive(Default)]
pub struct CreatorDashboardView {
    game_id: String,
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

        if ui.button("Manage Game").clicked() {
            println!("TODO!");
        }

        let game_id = self.game_id.parse();

        if ui.button("Upload Game").clicked() {
            if let Ok(game_id) = game_id {
                if let Some(file) = FileDialog::new()
                    .add_filter("gcrom (.gcrom)", &["gcrom"])
                    .pick_file()
                {
                    let bytes = std::fs::read(file).unwrap();

                    context.task_manager.rom.try_upload_rom(
                        UploadRom { game_id, bytes },
                        &context.auth_state.get_session().unwrap(),
                    )
                }
            }
        }
    }
}
