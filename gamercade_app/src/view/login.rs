use eframe::egui::{TextEdit, Ui};

use crate::task_manager::SuperTaskManager;

use super::ActiveView;

#[derive(Default)]
pub struct LoginView {
    provider: String,
    provider_kind: Provider,
    password: String,
}

#[derive(Default, PartialEq, Eq)]
enum Provider {
    #[default]
    Username,
    Email,
}

impl LoginView {
    pub fn draw(&mut self, ui: &mut Ui, task_manager: &mut SuperTaskManager) -> Option<ActiveView> {
        ui.horizontal(|ui| {
            let text = match self.provider_kind {
                Provider::Username => "Username: ",
                Provider::Email => "Email: ",
            };
            ui.label(text);
            ui.text_edit_singleline(&mut self.provider);
        });

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.provider_kind, Provider::Username, "Username");
            ui.separator();
            ui.selectable_value(&mut self.provider_kind, Provider::Email, "Email");
        });

        ui.horizontal(|ui| {
            ui.label("Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password).password(true);
            ui.add(pw_entry);
        });

        if ui.button("Sign Up").clicked() {
            return Some(ActiveView::sign_up());
        }

        if ui.button("Login").clicked() {
            // TODO: Support email login too
            task_manager.auth.try_login(&self.provider, &self.password);
            //TODO: Lock entries while waiting
            //TODO: Show an animation thing
        }

        if ui.button("Continue offline").clicked() {
            return Some(ActiveView::offline_browsing());
        }

        None
    }
}
