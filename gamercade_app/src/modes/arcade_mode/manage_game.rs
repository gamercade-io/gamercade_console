use rfd::FileDialog;

use crate::{app::AppDrawContext, local_directory::Game, task_manager::UploadRom};

#[derive(Default)]
pub struct ManageGameView {
    awaiting_upload: bool,
    game_id: i64,
    title: String,
    short_description: String,
    long_description: String,
    tags: Vec<i32>,
}

impl ManageGameView {
    pub fn new(game: &Game) -> Self {
        Self {
            awaiting_upload: false,
            game_id: game.id,
            title: game.title.clone(),
            short_description: game.short_description.clone(),
            long_description: game.long_description.as_ref().cloned().unwrap_or_default(),
            tags: game.tags.clone(),
        }
    }

    pub fn draw(&mut self, context: &mut AppDrawContext) -> bool {
        let mut done = false;
        let ui = &mut context.ui;

        ui.set_enabled(!self.awaiting_upload);

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

        if ui.button("Upload Game").clicked() {
            if let Some(file) = FileDialog::new()
                .add_filter("gcrom (.gcrom)", &["gcrom"])
                .pick_file()
            {
                let bytes = std::fs::read(file).unwrap();

                context.task_manager.rom.try_upload_rom(
                    UploadRom {
                        game_id: self.game_id,
                        bytes,
                    },
                    &context.auth_state.get_session().unwrap(),
                )
            }
        }

        if self.awaiting_upload {
            ui.spinner();
        }

        if ui.button("Go Back").clicked() {
            done = true;
        }

        done
    }
}
