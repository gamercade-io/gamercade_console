use crate::app::AppDrawContext;

use super::{ArcadeActiveView, CreatorDashboardView};

#[derive(Default)]
pub struct OnlineView {
    active_mode: OnlineViewMode,

    pub arcade: ArcadeView,
    pub dashboard: CreatorDashboardView,
}

#[derive(Default)]
pub struct ArcadeView {}

impl ArcadeView {
    fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;
        ui.horizontal(|ui| {});
    }
}

#[derive(PartialEq, Default)]
pub enum OnlineViewMode {
    #[default]
    Arcade,
    CreatorDashboard,
}

impl OnlineView {
    pub fn draw(&mut self, ctx: &mut AppDrawContext) -> Option<ArcadeActiveView> {
        ctx.ui.label("Online View");

        ctx.ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_mode, OnlineViewMode::Arcade, "Arcade");
            ui.selectable_value(
                &mut self.active_mode,
                OnlineViewMode::CreatorDashboard,
                "Creator Dashboard",
            );
        });

        match self.active_mode {
            OnlineViewMode::Arcade => self.arcade.draw(ctx),
            OnlineViewMode::CreatorDashboard => self.dashboard.draw(ctx),
        }

        if ctx.ui.button("Back").clicked() {
            return Some(ArcadeActiveView::login());
        }

        None
    }
}
