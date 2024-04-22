use std::process::Command;

use eframe::egui;

use crate::{
    app::AppDrawContext,
    game_rom_path,
    local_directory::{IsDictionary, TagId},
};

#[derive(Default)]
pub struct LibraryModeView {}

impl LibraryModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let AppDrawContext { ui, directory, .. } = context;

        let tag_directory = directory.tags.get_map();

        ui.label("Library Mode");

        let image_source = egui::include_image!("./../../../default-logo.png");
        let image = egui::Image::new(image_source).fit_to_exact_size((100.0, 100.0).into());

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("game_grid")
                .num_columns(6)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // Image, Title, Size, Short Description, Tags
                    ui.label("Game");
                    ui.label("Title");
                    ui.label("Size (mb)");
                    ui.label("Short Description");
                    ui.label("Tags");
                    ui.end_row();

                    for game in directory.iter_games() {
                        ui.add(image.clone());

                        ui.label(&game.title);

                        let rom_size_text = if let Some(rom_size) = game.rom_size {
                            let rom_size = rom_size as f32 / (1024.0 * 1024.0);
                            format!("{rom_size}")
                        } else {
                            String::new()
                        };

                        ui.label(rom_size_text);

                        ui.label(&game.short_description);

                        let tags = game
                            .tags
                            .iter()
                            .flat_map(|tag| {
                                tag_directory.get(&TagId(*tag)).map(|tag| tag.0.clone())
                            })
                            .collect::<Vec<_>>()
                            .join(",");
                        ui.label(tags);

                        if game.file_checksum.is_some() && game.rom_size.is_some() {
                            if ui.button("Play").clicked() {
                                println!("Play: {} ({})", game.title, game.id);

                                let mut command = Command::new("gccl");

                                command
                                    .arg("console")
                                    .arg("rom")
                                    .arg(game_rom_path(game.id));

                                println!("command: {command:?}");

                                if let Err(e) = command.spawn() {
                                    println!("Error launching game {e}");
                                }
                            }
                        } else {
                            ui.label("ROM Missing");
                        }
                        ui.end_row()
                    }
                });
        });
    }
}
