use eframe::egui::{self, TextEdit};

use crate::auth::AuthClient;

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Do this
            //     ui.horizontal(|ui| {
            //         ui.label("Username: ");
            //         ui.text_edit_singleline(&mut login_page.username);
            //     });

            //     ui.horizontal(|ui| {
            //         ui.label("Password: ");
            //         let pw_entry = TextEdit::singleline(&mut login_page.password).password(true);
            //         ui.add(pw_entry);
            //     });

            //     if ui.button("Login").clicked() {
            //         println!("Tried to login.");
            //     }
            // },
        });
    }
}
