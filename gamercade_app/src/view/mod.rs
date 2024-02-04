use eframe::egui::Ui;

use crate::{auth::AuthClient, local_directory::LocalDirectory};

use self::{browsing::BrowsingView, login::LoginView, sign_up::SignUpView};

mod browsing;
mod login;
mod sign_up;

pub enum ActiveView {
    Login(LoginView),
    SignUp(SignUpView),
    Browsing(BrowsingView),
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

    fn browsing() -> Self {
        Self::Browsing(BrowsingView::default())
    }

    pub fn draw(&mut self, ui: &mut Ui, auth_client: &mut AuthClient, directory: &LocalDirectory) {
        if let Some(next) = match self {
            ActiveView::Login(view) => view.draw(ui, auth_client),
            ActiveView::SignUp(view) => view.draw(ui, auth_client),
            ActiveView::Browsing(view) => view.draw(ui, directory),
        } {
            *self = next;
        }
    }
}
