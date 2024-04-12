use crate::app::AppDrawContext;

mod creator_dashboard;
mod edit_game;
mod login;
mod online_browsing;
mod sign_up;

use creator_dashboard::*;
use edit_game::*;
use login::*;
use online_browsing::*;
use sign_up::*;

#[derive(Default)]
pub struct ArcadeModeView {
    active_view: ArcadeActiveView,
}

impl ArcadeModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
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
    OnlineBrowsing(OnlineBrowsingView),
    CreatorDashboard(CreatorDashboardView),
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
        Self::OnlineBrowsing(OnlineBrowsingView::default())
    }

    fn draw(&mut self, context: AppDrawContext) {
        if let Some(next) = match self {
            ArcadeActiveView::Login(view) => view.draw(context),
            ArcadeActiveView::SignUp(view) => view.draw(context),
            ArcadeActiveView::OnlineBrowsing(view) => view.draw(context),
            ArcadeActiveView::CreatorDashboard(view) => view.draw(context),
        } {
            *self = next;
        }
    }
}
