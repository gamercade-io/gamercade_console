use std::path::PathBuf;

use eframe::egui::{self, menu, Context};
use rfd::FileDialog;

use gamercade_fs::EditorRom;

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
        let rom = EditorRom::default();
        Self {
            mode: EditorMode::Rom,
            rom_editor: RomEditor::default(),
            graphics_editor: GraphicsEditor::default(),
            audio_editor: AudioEditor::new(&rom.sounds),
            wasm_path: None,
            rom,
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.draw_menu_panel(ctx);
        self.draw_bottom_panel(ctx);
        self.draw_central_panel(ctx);
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
                        self.audio_editor.audio_sync_helper.notify_rom_changed();
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
    if let Some(path) = FileDialog::new()
        .add_filter("gce (.gce)", &["gce"])
        .pick_file()
    {
        match EditorRom::try_load(&path) {
            Ok(new_rom) => {
                *rom = new_rom;
                return Ok(());
            }
            Err(_) => return Err("Failed to load editor rom."),
        }
    }

    Ok(())
}

fn try_save_editor_rom(rom: &EditorRom) -> Result<(), &'static str> {
    if let Some(path) = FileDialog::new()
        .add_filter("gce (.gce)", &["gce"])
        .save_file()
    {
        rom.try_save(&path)
            .map_err(|_| "Failed to save editor rom.")
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
        let wasm = gamercade_fs::try_load_wasm(path).map_err(|_| "failed to read as bytes")?;

        if let Some(path) = FileDialog::new()
            .add_filter("gcrom (.gcrom)", &["gcrom"])
            .set_title("Export Game .gcrom")
            .save_file()
        {
            let rom = gamercade_fs::bundle(&wasm, rom);
            rom.try_save(&path)
                .map_err(|_| "failed to finish writing")?;
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
