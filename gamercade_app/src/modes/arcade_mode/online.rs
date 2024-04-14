use std::default;

use crate::app::AppDrawContext;

use super::{ArcadeActiveView, CreatorDashboardView};

#[derive(Default)]
pub struct OnlineView {
    release_id: String,
    active_mode: OnlineViewMode,

    arcade: ArcadeView,
    dashboard: CreatorDashboardView,
}

#[derive(Default)]
pub struct ArcadeView {
    release_id: String,
}

impl ArcadeView {
    fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;
        ui.horizontal(|ui| {
            ui.label("Release Id: ");
            ui.text_edit_singleline(&mut self.release_id);
            if ui.button("Download Release").clicked() {
                println!("TODO: Download Release")
            }
        });
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
