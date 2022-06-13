use std::fs;

use egui::{self, menu, Context};
use rfd::FileDialog;

use crate::editor_data::EditorRom;

use super::{GraphicsEditor, SoundsEditor};

pub struct Editor {
    pub rom: EditorRom,
    pub mode: EditorMode,

    graphics_editor: GraphicsEditor,
    sounds_editor: SoundsEditor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorMode {
    GraphicsMode,
    SoundMode,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mode: EditorMode::GraphicsMode,
            rom: EditorRom::default(),
            graphics_editor: GraphicsEditor::default(),
            sounds_editor: SoundsEditor::default(),
        }
    }
}

impl Editor {
    pub fn draw_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("editor_top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("TODO: new file!");
                        ui.close_menu();
                    }

                    if ui.button("Open").clicked() {
                        if let Err(e) = try_load_editor_rom(&mut self.rom) {
                            println!("{}", e);
                        }
                        ui.close_menu();
                    }

                    if ui.button("Save").clicked() {
                        if let Err(e) = try_save_editor_rom(&self.rom) {
                            println!("{}", e);
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("Game", |ui| {
                    if ui.button("Local Test Game").clicked() {
                        println!("TODO: Test Local Game!");
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Export Game").clicked() {
                        // TODO: Write this to a file somewhere...
                        let _output = self.rom.export_as_rom();
                        ui.close_menu();
                    }
                })
            });
        });
    }

    pub fn draw_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, EditorMode::GraphicsMode, "Graphics Mode");

                ui.selectable_value(&mut self.mode, EditorMode::SoundMode, "Sounds Mode");

                ui.separator();

                ui.horizontal(|ui| match &mut self.mode {
                    EditorMode::GraphicsMode => self.graphics_editor.draw_selector(ui),
                    EditorMode::SoundMode => self.sounds_editor.draw_selector(ui),
                });
            });

            match self.mode {
                EditorMode::GraphicsMode => self
                    .graphics_editor
                    .draw_contents(ui, &mut self.rom.graphics),
                EditorMode::SoundMode => self.sounds_editor.draw_contents(ui, &mut self.rom.sounds),
            }
        });
    }

    pub fn draw_bottom_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("editor_bottom_panel").show(ctx, |ui| match self.mode {
            EditorMode::GraphicsMode => self.graphics_editor.draw_bottom_panel(ui),
            EditorMode::SoundMode => self.sounds_editor.draw_bottom_panel(ui),
        });
    }
}

fn try_load_editor_rom(rom: &mut EditorRom) -> Result<(), &'static str> {
    let text = if let Some(path) = FileDialog::new()
        .add_filter("gce (.gce)", &["gce"])
        .pick_file()
    {
        match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => return Err("failed to read file to string."),
        }
    } else {
        return Err("failed to read file");
    };

    match serde_json::from_str::<EditorRom>(&text) {
        Ok(parsed) => {
            *rom = parsed;
            Ok(())
        }
        Err(_) => Err("failed to parse text from file"),
    }
}

fn try_save_editor_rom(rom: &EditorRom) -> Result<(), &'static str> {
    if let Some(path) = FileDialog::new()
        .add_filter("gce (.gce)", &["gce"])
        .save_file()
    {
        fs::write(path, serde_json::to_string(rom).unwrap()).map_err(|_| "failed to write file")
    } else {
        Err("failed to find save file location")
    }
}
