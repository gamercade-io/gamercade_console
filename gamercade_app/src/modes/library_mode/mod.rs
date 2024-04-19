use std::process::Command;

use eframe::egui;

use crate::{app::AppDrawContext, game_rom_path};

#[derive(Default)]
pub struct LibraryModeView {}

impl LibraryModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let AppDrawContext { ui, directory, .. } = context;
        ui.label("Library Mode");

        egui::Grid::new("game_grid")
            .num_columns(4)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // Title, Short Description
                ui.label("Title");
                ui.label("Short Description");
                ui.end_row();

                for game in directory.iter_games() {
                    ui.label(&game.title);
                    ui.label(&game.short_description);
                    if ui.button("Play").clicked() {
                        println!("Play: {} ({})", game.title, game.id);

                        let mut command = Command::new("gccl");

                        // TODO: Fix paths

                        command
                            .arg("console")
                            .arg("rom")
                            .arg(game_rom_path(game.id));

                        println!("command: {command:?}");

                        if let Err(e) = command.spawn() {
                            println!("Error launching game {e}");
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
