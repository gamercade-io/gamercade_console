use eframe::egui::Ui;

use crate::{local_directory::LocalDirectory, task_manager::SuperTaskManager};

use self::{
    login::LoginView, offline_browsing::OfflineBrowsingView, online_browsing::OnlineBrowsingView,
    sign_up::SignUpView,
};

mod login;
mod offline_browsing;
mod online_browsing;
mod sign_up;

pub enum ActiveView {
    Login(LoginView),
    SignUp(SignUpView),
    OfflineBrowsing(OfflineBrowsingView),
    OnlineBrowsing(OnlineBrowsingView),
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

    pub fn draw(
        &mut self,
        ui: &mut Ui,
        tasks: &mut SuperTaskManager,
        directory: &mut LocalDirectory,
    ) {
        if let Some(next) = match self {
            ActiveView::Login(view) => view.draw(ui, tasks),
            ActiveView::SignUp(view) => view.draw(ui, tasks),
            ActiveView::OfflineBrowsing(view) => view.draw(ui, directory),
            ActiveView::OnlineBrowsing(view) => view.draw(ui, directory),
        } {
            *self = next;
        }
    }
}
