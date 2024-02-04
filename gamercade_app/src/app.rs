use eframe::egui::{self};

use crate::{auth::AuthClient, local_directory::LocalDirectory, view::ActiveView};

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    active_view: ActiveView,

    directory: LocalDirectory,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.active_view
                .draw(ui, &mut self.auth_client, &self.directory);
        });
    }
}
