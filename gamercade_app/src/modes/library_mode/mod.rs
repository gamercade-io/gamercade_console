use std::process::Command;

use eframe::egui::{self, Image};

use crate::{
    app::AppDrawContext,
    game_rom_path,
    local_directory::{ImageCache, IsDictionary, TagId},
};

#[derive(Default)]
pub struct LibraryModeView {}

impl LibraryModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let tag_directory = context.directory.tags.get_map();

        context.ui.label("Library Mode");

        egui::ScrollArea::vertical().show(context.ui, |ui| {
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

                    for game in context.directory.iter_games() {
                        let image =
                            if let Some(image) = context.directory.images.games.get(&game.id) {
                                Image::new(image)
                            } else {
                                Image::new(ImageCache::default_game_image().clone())
                            };
                        ui.add(image.fit_to_exact_size((100.0, 100.0).into()));

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

    pub fn sync_against_fs(&mut self) -> Vec<FsGame> {
        //TODO: This function
        // Check against FS for existing .gcroms
        // If they exist and... 
        // 1. ROM is in localDB, do nothing
        // 2. ROM is not in localDB, fetch the update by ID

        Vec::default()
    }
}

pub enum FsGame {}
