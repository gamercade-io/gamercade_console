use eframe::egui::Window;
use tokio::task::block_in_place;

use crate::{app::AppDrawContext, task_manager::DownloadStatus};

#[derive(Default)]
pub struct DownloadWindow {
    pub open: bool,
}

impl DownloadWindow {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        Window::new("Downloads")
            .open(&mut self.open)
            .collapsible(false)
            .resizable(false)
            .show(context.ui.ctx(), |ui| {
                ui.spinner();
                block_in_place(|| {
                    let lock = context.task_manager.http.state.blocking_lock();

                    lock.rom_downloads.iter().for_each(|(key, value)| {
                        let game_name = context
                            .directory
                            .cached_games
                            .iter()
                            .find(|game| game.id == *key)
                            .map(|game| game.title.as_str())
                            .unwrap_or("Unknown Game");

                        match value.download_status {
                            DownloadStatus::Starting => {
                                ui.label(format!("{game_name}: Starting..."));
                            }
                            DownloadStatus::InProgress {
                                bytes_downloaded,
                                total_bytes,
                            } => {
                                ui.label(format!(
                                    "{game_name}: {}",
                                    bytes_downloaded as f32 / total_bytes as f32
                                ));
                            }
                            DownloadStatus::Done(_) => {
                                ui.label(format!("{game_name}: Done"));
                            }
                        }
                    });
                });
            });
    }
}
