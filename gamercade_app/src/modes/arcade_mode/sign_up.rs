use eframe::egui::{Button, TextEdit};

use crate::app::AppDrawContext;

use super::ArcadeActiveView;

#[derive(Default)]
pub struct SignUpView {
    username: String,
    password: String,
    password_confirm: String,
    email: String,
    email_confirm: String,
}

impl SignUpView {
    pub fn draw(&mut self, context: &mut AppDrawContext) -> Option<ArcadeActiveView> {
        let AppDrawContext {
            ui, task_manager, ..
        } = context;

        let mut email_equal = false;
        let mut password_equal = false;

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
            ui.label("Re-Enter Email Address: ");
            let email = TextEdit::singleline(&mut self.email_confirm);
            ui.add(email);

            if self.email != self.email_confirm {
                ui.label("Email doesn't match.");
            } else {
                email_equal = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password).password(true);
            ui.add(pw_entry);
        });

        ui.horizontal(|ui| {
            ui.label("Re-Enter Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password_confirm).password(true);
            ui.add(pw_entry);

            if self.password != self.password_confirm {
                ui.label("Password doesn't match.");
            } else {
                password_equal = true;
            }
        });

        if ui
            .add_enabled(email_equal && password_equal, Button::new("Register"))
            .clicked()
        {
            task_manager
                .auth
                .try_register(&self.username, &self.email, &self.password);
            // TODO: Lock the entries while waiting...
            // TODO: Show an animation thing...
        }

        if ui.button("Cancel").clicked() {
            Some(ArcadeActiveView::login())
        } else {
            None
        }
    }
}
