use gamercade_interface::game::UpdateGameRequest;
use rfd::FileDialog;

use crate::{
    app::AppDrawContext,
    task_manager::{GameRequest, UploadRom},
    urls::WithSession,
};

use super::new_game::NewGameView;

#[derive(Default)]
pub struct CreatorDashboardView {
    pub view: DashboardView,
    pub new_game_view: NewGameView,
}

#[derive(Default)]
enum DashboardView {
    #[default]
    Main,
    NewGameView,
}

impl CreatorDashboardView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let enabled = context.auth_state.get_session().is_some();

        context.ui.set_enabled(enabled);

        context.ui.label("Creator Dashboard");

        match self.view {
            DashboardView::Main => {
                self.draw_main_view(context);
            }
            DashboardView::NewGameView => {
                if self.new_game_view.draw(context) {
                    self.view = DashboardView::Main
                }
            }
        }

        // ui.separator();

        // ui.label("Game Id:");
        // ui.text_edit_singleline(&mut self.game_id);

        // if ui.button("Manage Game").clicked() {
        //     println!("TODO!");
        // }

        // let game_id = self.game_id.parse();

        // if ui.button("Upload Game").clicked() {
        //     if let Ok(game_id) = game_id {
        //         if let Some(file) = FileDialog::new()
        //             .add_filter("gcrom (.gcrom)", &["gcrom"])
        //             .pick_file()
        //         {
        //             let bytes = std::fs::read(file).unwrap();

        //             context.task_manager.rom.try_upload_rom(
        //                 UploadRom { game_id, bytes },
        //                 &context.auth_state.get_session().unwrap(),
        //             )
        //         }
        //     }
        // }
    }

    fn draw_main_view(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        ui.separator();
        if ui.button("Create New Game").clicked() {
            self.view = DashboardView::NewGameView;
        }
    }
}
