use std::{fs, io::Write, path::PathBuf};

use egui::{self, menu, Context};
use rfd::FileDialog;

use crate::editor_data::EditorRom;

use super::{AudioEditor, GraphicsEditor, RomEditor};

pub struct Editor {
    pub rom: EditorRom,
    pub mode: EditorMode,

    rom_editor: RomEditor,
    graphics_editor: GraphicsEditor,
    audio_editor: AudioEditor,

    wasm_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorMode {
    Rom,
    Graphics,
    Audio,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mode: EditorMode::Rom,
            rom: EditorRom::default(),
            rom_editor: RomEditor::default(),
            graphics_editor: GraphicsEditor::default(),
            audio_editor: AudioEditor::default(),
            wasm_path: None,
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
                    if ui.button("Select game .wasm").clicked() {
                        if let Err(e) = try_select_wasm(&mut self.wasm_path) {
                            println!("{}", e);
                        };
                        ui.close_menu();
                    }

                    if ui.button("Export Game").clicked() {
                        if let Err(e) = try_export_rom(&self.rom, &mut self.wasm_path) {
                            println!("{}", e);
                        }
                        ui.close_menu();
                    }
                })
            });
        });
    }

    pub fn draw_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, EditorMode::Rom, "Rom Settings");
                ui.selectable_value(&mut self.mode, EditorMode::Graphics, "Graphics Mode");
                ui.selectable_value(&mut self.mode, EditorMode::Audio, "Audio Mode");

                ui.separator();

                ui.horizontal(|ui| match &mut self.mode {
                    EditorMode::Rom => (),
                    EditorMode::Graphics => self.graphics_editor.draw_selector(ui),
                    EditorMode::Audio => self.audio_editor.draw_selector(ui),
                });
            });

            match self.mode {
                EditorMode::Rom => self.rom_editor.draw_contents(ui, &mut self.rom),
                EditorMode::Graphics => self
                    .graphics_editor
                    .draw_contents(ui, &mut self.rom.graphics),
                EditorMode::Audio => self.audio_editor.draw_contents(ui, &mut self.rom.sounds),
            }
        });
    }

    pub fn draw_bottom_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("editor_bottom_panel").show(ctx, |ui| match self.mode {
            EditorMode::Rom => (),
            EditorMode::Graphics => self.graphics_editor.draw_bottom_panel(ui),
            EditorMode::Audio => self.audio_editor.draw_bottom_panel(ui),
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
        Err(e) => {
            println!("{}", e);
            Err("failed to parse text from file")
        }
    }
}

fn try_save_editor_rom(rom: &EditorRom) -> Result<(), &'static str> {
    if let Some(path) = FileDialog::new()
        .add_filter("gce (.gce)", &["gce"])
        .save_file()
    {
        fs::write(
            path,
            serde_json::to_string_pretty(rom).expect("failed to serialize editor rom to json"),
        )
        .map_err(|_| "failed to write file")
    } else {
        Ok(())
    }
}

fn try_select_wasm(wasm_path: &mut Option<PathBuf>) -> Result<(), &'static str> {
    match try_pick_wasm() {
        Some(path) => {
            *wasm_path = Some(path);
        }
        None => return Err("didn't select a .wasm file"),
    };

    Ok(())
}

fn try_export_rom(rom: &EditorRom, wasm_path: &mut Option<PathBuf>) -> Result<(), &'static str> {
    *wasm_path = match wasm_path {
        Some(path) => Some(path.to_path_buf()),
        None => match try_pick_wasm() {
            Some(path) => Some(path),
            None => return Err("didn't select a .wasm file"),
        },
    };

    if let Some(path) = wasm_path {
        let wasm = fs::read(path).map_err(|_| "failed to read as bytes")?;

        if let Some(path) = FileDialog::new()
            .add_filter("gcrom (.gcrom)", &["gcrom"])
            .set_title("Export Game .gcrom")
            .save_file()
        {
            let rom = rom.export_as_rom(&wasm);
            let rom =
                bincode::serialize(&rom).map_err(|_| "failed to serialize editor rom to binary")?;
            let target = fs::File::create(path).map_err(|_| "failed to create file")?;
            let mut encoder = zstd::Encoder::new(target, zstd::DEFAULT_COMPRESSION_LEVEL)
                .map_err(|_| "failed to create encoder")?;

            encoder
                .write_all(&rom)
                .map_err(|_| "failed to encoder.write")?;

            encoder.finish().map_err(|_| "failed to finish writing")?;
        }
    }

    Ok(())
}

fn try_pick_wasm() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("wasm (.wasm)", &["wasm"])
        .set_title("Load .wasm file")
        .pick_file()
}
