use eframe::egui::{self, ViewportBuilder};

mod app;
mod local_directory;
mod modes;
mod task_manager;
mod urls;

pub const GAME_DIR: &str = "./roms";
pub const IMAGE_DIR: &str = "./images";

pub fn game_rom_path(game_id: i64) -> String {
    format!("{GAME_DIR}/{game_id:x}.gcrom")
}

pub fn game_image_path(game_id: i64) -> String {
    format!("{GAME_DIR}/{game_id:x}.png")
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let viewport = ViewportBuilder::default().with_inner_size(egui::vec2(1280.0, 720.0));

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Gamercade Platform",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(app::App::new(&cc.egui_ctx))
        }),
    )
}
