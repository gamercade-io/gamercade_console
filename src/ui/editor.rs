use eframe::egui;
use eframe::egui::{menu, Context};
use gamercade_core::Rom;

use crate::editor_data::EditorRom;

use super::{GraphicsEditor, GraphicsEditorMode, SoundsEditor};

pub struct Editor {
    pub rom: Rom,
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
            rom: Rom::default(),
            graphics_editor: GraphicsEditor::default(),
            sounds_editor: SoundsEditor::default(),
        }
    }
}

impl Editor {
    pub fn draw_menu_panel(&self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("TODO: new file!");
                        ui.close_menu();
                    }

                    if ui.button("Open").clicked() {
                        println!("TODO: Open file!");
                        ui.close_menu();
                    }

                    if ui.button("Save").clicked() {
                        println!("TODO: Save file!");
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
                        println!("TODO: Export game!");
                        ui.close_menu();
                    }
                })
            });
        });
    }

    pub fn draw_modes_buttons(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, EditorMode::GraphicsMode, "Graphics Editor");

                ui.selectable_value(&mut self.mode, EditorMode::SoundMode, "Sounds Editor");
            });

            ui.horizontal(|ui| match &mut self.mode {
                EditorMode::GraphicsMode => self.graphics_editor.draw(ui),
                EditorMode::SoundMode => self.sounds_editor.draw(ui),
            });
        });
    }
}
