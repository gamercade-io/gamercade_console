use crate::app::AppDrawContext;

mod creator_dashboard;
mod login;
mod new_game;
mod online;
mod sign_up;

use creator_dashboard::*;
use login::*;
use online::*;
use sign_up::*;

#[derive(Default)]
pub struct ArcadeModeView {
    active_view: ArcadeActiveView,
}

impl ArcadeModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        context.ui.label("Arcade Mode View");

        self.active_view.draw(context);
    }

    pub fn logged_in(&mut self) {
        self.active_view = ArcadeActiveView::online_browsing()
    }

    pub fn logged_out(&mut self) {
        self.active_view = ArcadeActiveView::login()
    }
}

pub enum ArcadeActiveView {
    Login(LoginView),
    SignUp(SignUpView),
    Online(OnlineView),
}

impl Default for ArcadeActiveView {
    fn default() -> Self {
        Self::login()
    }
}

impl ArcadeActiveView {
    fn login() -> Self {
        Self::Login(LoginView::default())
    }

    fn sign_up() -> Self {
        Self::SignUp(SignUpView::default())
    }

    fn online_browsing() -> Self {
        Self::Online(OnlineView::default())
    }

    fn draw(&mut self, context: &mut AppDrawContext) {
        if let Some(next) = match self {
            ArcadeActiveView::Login(view) => view.draw(context),
            ArcadeActiveView::SignUp(view) => view.draw(context),
            ArcadeActiveView::Online(view) => view.draw(context),
        } {
            *self = next;
        }
    }
}
