use eframe::egui;
use gamercade_interface::PERMISSION_LEVEL_EDITOR;

use crate::{
    app::AppDrawContext,
    local_directory::{GameId, IsDictionary},
};

use super::{manage_game::ManageGameView, new_game::NewGameView};

#[derive(Default)]
pub struct CreatorDashboardView {
    view: DashboardView,
    pub new_game_view: NewGameView,
    pub manage_game_view: ManageGameView,
}

#[derive(Default)]
enum DashboardView {
    #[default]
    Main,
    NewGameView,
    ManageGameView,
}

impl CreatorDashboardView {
    pub fn main_view(&mut self) {
        self.view = DashboardView::Main
    }

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
            DashboardView::ManageGameView => {
                if self.manage_game_view.draw(context) {
                    self.view = DashboardView::Main
                }
            }
        }
    }

    fn draw_main_view(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        if ui.button("PRINT FOOTPRINT").clicked() {
            println!("{:?}", context.directory.game_footprint.get_map());
        }

        egui::Grid::new("editable_grid")
            .num_columns(3)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Editable Games:");
                ui.end_row();

                ui.label("Title");
                ui.label("Rom Details");
                ui.label("Manage");
                ui.end_row();

                // Only iterate games which we are able to edit
                for game in context.directory.iter_games().filter(|game| {
                    context
                        .directory
                        .game_footprint
                        .get_map()
                        .get(&GameId(game.id))
                        .map(|game| {
                            if let Some(level) = game.permission_level {
                                level <= PERMISSION_LEVEL_EDITOR
                            } else {
                                false
                            }
                        })
                        .unwrap_or_default()
                }) {
                    ui.label(&game.title);

                    let rom_exists =
                        if let (Some(_), Some(size)) = (game.file_checksum, game.rom_size) {
                            let size = size as f32 / (1024.0 * 1024.0);
                            format!("{size}mb")
                        } else {
                            format!("N/A")
                        };
                    ui.label(rom_exists);

                    if ui.button("Manage Game").clicked() {
                        self.manage_game_view = ManageGameView::new(game);
                        self.view = DashboardView::ManageGameView;
                    }

                    ui.end_row();
                }
            });

        ui.separator();
        if ui.button("Create New Game").clicked() {
            self.view = DashboardView::NewGameView;
        }
    }
}
