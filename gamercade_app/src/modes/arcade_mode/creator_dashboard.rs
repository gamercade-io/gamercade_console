use gamercade_interface::game::UpdateGameRequest;

use crate::{app::AppDrawContext, task_manager::GameRequest, urls::WithSession};

#[derive(Default)]
pub struct CreatorDashboardView {}

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

        if ui.button("Manage Game").clicked() {}

        if ui.button("Create Release").clicked() {}
    }
}
