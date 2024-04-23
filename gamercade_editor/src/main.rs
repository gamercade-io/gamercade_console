use eframe::{egui::ViewportBuilder, epaint::Vec2};
use ui::Editor;

mod ui;

fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1366.0, 768.0));

    let options = eframe::NativeOptions {
        vsync: true,
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Gamercade Editor",
        options,
        Box::new(|_cc| Box::<Editor>::default()),
    )
    .expect("Failed to run window");
}
