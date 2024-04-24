use crate::app::AppDrawContext;

mod creator_dashboard;
mod download_window;
mod login;
mod manage_game;
mod new_game;
mod online;
mod sign_up;

use creator_dashboard::*;
use login::*;
use online::*;
use sign_up::*;

#[derive(Default)]
pub struct ArcadeModeView {
    pub active_view: ArcadeActiveView,

    pub login: LoginView,
    pub sign_up: SignUpView,
    pub online: OnlineView,
}

impl ArcadeModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        context.ui.label("Arcade Mode View");

        if let Some(next) = match self.active_view {
            ArcadeActiveView::Login => self.login.draw(context),
            ArcadeActiveView::SignUp => self.sign_up.draw(context),
            ArcadeActiveView::Online => self.online.draw(context),
        } {
            self.active_view = next;
        }
    }

    pub fn logged_in(&mut self) {
        self.active_view = ArcadeActiveView::online_browsing()
    }

    pub fn logged_out(&mut self) {
        self.active_view = ArcadeActiveView::login()
    }
}

pub enum ArcadeActiveView {
    Login,
    SignUp,
    Online,
}

impl Default for ArcadeActiveView {
    fn default() -> Self {
        Self::login()
    }
}

impl ArcadeActiveView {
    fn login() -> Self {
        Self::Login
    }

    fn sign_up() -> Self {
        Self::SignUp
    }

    fn online_browsing() -> Self {
        Self::Online
    }
}
