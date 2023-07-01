use eframe::egui::{self, TextEdit};

use crate::auth::AuthClient;

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    username: String,
    password: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Do this
            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.username);
            });

            ui.horizontal(|ui| {
                ui.label("Password: ");
                let pw_entry = TextEdit::singleline(&mut self.password).password(true);
                ui.add(pw_entry);
            });

            if ui.button("Login").clicked() {
                self.auth_client.try_login(&self.username, &self.password);
                self.username.clear();
                self.password.clear();
            }
            // },
        });
    }
}
