use editor_state::{EditorMode, EditorState};
use eframe::{egui, epi};

mod editor_state;

impl epi::App for editor_state::EditorState {
    fn name(&self) -> &str {
        "Gamercade Editor"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // TODO:
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

                if ui.button("Export Game").clicked() {
                    println!("TODO: Export game!");
                    ui.close_menu();
                }

                if ui.button("Close").clicked() {
                    ui.close_menu()
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, EditorMode::PaletteEditor, "Palette Editor");
                ui.selectable_value(
                    &mut self.mode,
                    EditorMode::SpriteSheetEditor,
                    "Sprite Sheet Editor",
                );
                ui.selectable_value(&mut self.mode, EditorMode::SpriteEditor, "Sprite Editor");
            });

            match self.mode {
                EditorMode::PaletteEditor => self.palette_editor(ui),
                EditorMode::SpriteSheetEditor => self.sprite_sheet_editor(ui),
                EditorMode::SpriteEditor => self.sprite_editor(ui),
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(EditorState::default()), options);
}
