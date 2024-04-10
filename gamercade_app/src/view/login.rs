use eframe::egui::TextEdit;

use crate::app::AppDrawContext;

use super::ActiveView;

#[derive(Default)]
pub struct LoginView {
    provider: String,
    provider_kind: Provider,
    password: String,
    waiting: bool,
}

#[derive(Default, PartialEq, Eq)]
enum Provider {
    #[default]
    Username,
    Email,
}

impl LoginView {
    pub fn draw(&mut self, context: AppDrawContext) -> Option<ActiveView> {
        let AppDrawContext {
            ui, task_manager, ..
        } = context;

        ui.set_enabled(!self.waiting);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.provider_kind, Provider::Username, "Username");
            ui.separator();
            ui.selectable_value(&mut self.provider_kind, Provider::Email, "Email");
        });

        ui.horizontal(|ui| {
            let text = match self.provider_kind {
                Provider::Username => "Username: ",
                Provider::Email => "Email: ",
            };
            ui.label(text);
            ui.text_edit_singleline(&mut self.provider);
        });

        ui.horizontal(|ui| {
            ui.label("Password: ");
            let pw_entry = TextEdit::singleline(&mut self.password).password(true);
            ui.add(pw_entry);
        });

        if ui.button("Login").clicked() {
            // TODO: Support email login too
            task_manager.auth.try_login(&self.provider, &self.password);
            self.waiting = true;
            //TODO: Lock entries while waiting
        }

        ui.separator();

        if ui.button("Sign Up").clicked() {
            return Some(ActiveView::sign_up());
        }

        if self.waiting {
            ui.spinner();
        }

        ui.set_enabled(true);
        if ui.button("Continue offline").clicked() {
            return Some(ActiveView::offline_browsing());
        }

        None
    }
}
