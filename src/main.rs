use eframe::{
    egui::{self},
    epi,
};

mod editor_data;
mod ui;

use ui::Editor;

impl epi::App for ui::Editor {
    fn name(&self) -> &str {
        "Gamercade Editor"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.draw_menu_panel(ctx);
        self.draw_modes_buttons(ctx);

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(Editor::default()), options);
}
