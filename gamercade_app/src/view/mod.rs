use eframe::egui::Ui;

use crate::{auth::AuthClient, local_directory::LocalDirectory};

use self::{login::LoginView, offline_browsing::OfflineBrowsingView, sign_up::SignUpView};

mod login;
mod offline_browsing;
mod sign_up;

pub enum ActiveView {
    Login(LoginView),
    SignUp(SignUpView),
    OfflineBrowsing(OfflineBrowsingView),
}

impl Default for ActiveView {
    fn default() -> Self {
        Self::login()
    }
}

impl ActiveView {
    fn login() -> Self {
        Self::Login(LoginView::default())
    }

    fn sign_up() -> Self {
        Self::SignUp(SignUpView::default())
    }

    fn offline_browsing() -> Self {
        Self::OfflineBrowsing(OfflineBrowsingView::default())
    }

    pub fn draw(&mut self, ui: &mut Ui, auth_client: &mut AuthClient, directory: &LocalDirectory) {
        if let Some(next) = match self {
            ActiveView::Login(view) => view.draw(ui, auth_client),
            ActiveView::SignUp(view) => view.draw(ui, auth_client),
            ActiveView::OfflineBrowsing(view) => view.draw(ui, directory),
        } {
            *self = next;
        }
    }
}
