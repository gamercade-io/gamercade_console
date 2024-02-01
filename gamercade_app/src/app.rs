use eframe::egui::{self};

use crate::{auth::AuthClient, view::ActiveView};

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    active_view: ActiveView,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.active_view.draw(ui, &mut self.auth_client);
        });
    }
}
