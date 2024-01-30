use eframe::egui::{self, TextEdit, Ui};

use crate::auth::AuthClient;

#[derive(Default)]
pub struct App {
    auth_client: AuthClient,

    username: String,
    password: String,
    email: String,

    active_view: ActiveView,
}

#[derive(Default)]
enum ActiveView {
    #[default]
    Login,
    SignUp,
    Browsing,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_view {
                ActiveView::Login => self.draw_login(ui),
                ActiveView::SignUp => self.draw_sign_up(ui),
                ActiveView::Browsing => self.draw_browsing(ui),
            };
        });
    }
}

impl App {
    fn draw_login(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Username: ");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.horizontal(|ui| {
            ui.label("Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password).password(true);
            ui.add(pw_entry);
        });

        if ui.button("Sign Up").clicked() {
            self.active_view = ActiveView::SignUp;
        }

        if ui.button("Login").clicked() {
            self.auth_client.try_login(&self.username, &self.password);
            //TODO: Lock entries while waiting
            //TODO: Show an animation thing
            self.clear_text();
        }

        if ui.button("Login as Guest").clicked() {
            println!("TODO: Login as guest!")
        }
    }

    fn draw_sign_up(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Username: ");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.horizontal(|ui| {
            ui.label("Email Address: ");
            let email = TextEdit::singleline(&mut self.email);
            ui.add(email);
        });

        ui.horizontal(|ui| {
            ui.label("Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password).password(true);
            ui.add(pw_entry);
        });

        if ui.button("Register").clicked() {
            self.auth_client
                .try_register(&self.username, &self.email, &self.password);
            // TODO: Lock the entries while waiting...
            // TODO: Show an animation thing...
            self.clear_text();
        }

        if ui.button("Cancel").clicked() {
            self.clear_text();
            self.active_view = ActiveView::Login;
        }
    }

    fn draw_browsing(&mut self, ui: &mut Ui) {
        ui.label("Browsing");
    }

    fn clear_text(&mut self) {
        self.username.clear();
        self.email.clear();
        self.password.clear();
    }
}
