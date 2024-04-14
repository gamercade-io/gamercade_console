use gamercade_interface::game::UpdateGameRequest;

use crate::{app::AppDrawContext, task_manager::GameRequest};

#[derive(Default)]
pub struct CreatorDashboardView {}

impl CreatorDashboardView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        ui.label("Creator Dashboard");

        if ui.button("Create Game").clicked() {
            context
                .task_manager
                .game
                .send(GameRequest::CreateGame(UpdateGameRequest {
                    game_id: None,
                    title: Some("Test Game".to_string()),
                    short_description: Some("A game for testing".to_string()),
                    long_description: Some("Some more details....".to_string()),
                }))
        }

        if ui.button("Manage Game").clicked() {}

        if ui.button("Create Release").clicked() {}
    }
}
