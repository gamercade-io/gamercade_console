use editor_state::{EditorMode, EditorState};
use eframe::{
    egui::{self, menu},
    epi,
};

use crate::graphics_editor::GraphicsEditorMode;

mod editor_state;
mod graphics_editor;

impl epi::App for editor_state::EditorState {
    fn name(&self) -> &str {
        "Gamercade Editor"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // TODO:
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| match &mut self.mode {
                EditorMode::GraphicsMode(graphics) => {
                    ui.selectable_value(
                        &mut graphics.mode,
                        GraphicsEditorMode::PaletteEditor,
                        "Palette Editor",
                    );
                    ui.selectable_value(
                        &mut graphics.mode,
                        GraphicsEditorMode::SpriteSheetEditor,
                        "Sprite Sheet Editor",
                    );
                    ui.selectable_value(
                        &mut graphics.mode,
                        GraphicsEditorMode::SpriteEditor,
                        "Sprite Editor",
                    );

                    match graphics.mode {
                        GraphicsEditorMode::PaletteEditor => graphics.palette_editor(ui),
                        GraphicsEditorMode::SpriteSheetEditor => graphics.sprite_sheet_editor(ui),
                        GraphicsEditorMode::SpriteEditor => graphics.sprite_editor(ui),
                    };
                }
                EditorMode::SoundMode => {
                    todo!()
                }
            });
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(EditorState::default()), options);
}
