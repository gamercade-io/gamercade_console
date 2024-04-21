use gamercade_interface::game::UpdateGameRequest;

use crate::{app::AppDrawContext, task_manager::GameRequest, urls::WithSession};

#[derive(Default, Debug)]
pub struct NewGameView {
    pub awaiting_game: bool,
    title: String,
    short_description: String,
    long_description: String,
}

impl NewGameView {
    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn draw(&mut self, context: &mut AppDrawContext) -> bool {
        let mut done = false;
        let ui = &mut context.ui;

        ui.set_enabled(!self.awaiting_game);

        ui.label("New Game View");
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
            if ui.button("Create Game").clicked() {
                self.awaiting_game = true;
                context
                    .task_manager
                    .game
                    .send(GameRequest::CreateGame(WithSession {
                        session: context.auth_state.get_session().unwrap(),
                        data: UpdateGameRequest {
                            game_id: None,
                            title: Some(self.title.clone()),
                            short_description: Some(self.short_description.clone()),
                            long_description: Some(self.long_description.clone()),
                        },
                    }));
            }

            ui.set_enabled(true);

            if ui.button("Go Back").clicked() {
                done = true
            }
        });

        if self.awaiting_game {
            ui.spinner();
        }

        done
    }
}
