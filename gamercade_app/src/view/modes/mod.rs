mod offline_mode;
mod online_mode;
mod settings_mode;

use eframe::egui::Ui;
pub use offline_mode::*;
pub use online_mode::*;
pub use settings_mode::*;

use crate::app::AppDrawContext;

pub enum AppModeTab {
    Online(OnlineModeView),
    Offline(OfflineModeView),
    Settings(SettingsView),
}

impl Default for AppModeTab {
    fn default() -> Self {
        Self::Online(OnlineModeView::default())
    }
}

impl AppModeTab {
    pub fn draw(&mut self, app: AppDrawContext) {
        match self {
            AppModeTab::Online(view) => view.draw(app),
            AppModeTab::Offline(view) => view.draw(app),
            AppModeTab::Settings(view) => view.draw(app),
        }
    }
}
