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
    game_id: String,
    release_id: String,
}

impl ArcadeView {
    fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;
        ui.horizontal(|ui| {
            ui.label("Game Id: ");
            ui.text_edit_singleline(&mut self.game_id);

            ui.label("Release Id: ");
            ui.text_edit_singleline(&mut self.release_id);
            if ui.button("Download Release").clicked() {
                let game_id = self.game_id.parse();
                let release_id = self.release_id.parse();

                if let (Ok(game_id), Ok(release_id)) = (game_id, release_id) {
                    context
                        .task_manager
                        .release
                        .try_download(game_id, release_id);
                } else {
                    self.release_id = String::new();
                    self.game_id = String::new();
                }
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
