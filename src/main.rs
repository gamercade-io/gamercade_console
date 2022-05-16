use eframe::{egui::Context, App, Frame};

mod editor_data;
mod ui;

use ui::Editor;

impl App for ui::Editor {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.draw_menu_panel(ctx);
        self.draw_central_panel(ctx);
        self.draw_bottom_panel(ctx);

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gamercade Editor",
        options,
        Box::new(|cc| Box::new(Editor::new(cc))),
    );
}
