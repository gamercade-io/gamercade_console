use eframe::egui;
use gamercade_interface::platform::FrontPageResponse;

use crate::app::AppDrawContext;

use super::{ArcadeActiveView, CreatorDashboardView};

#[derive(Default)]
pub struct OnlineView {
    active_mode: OnlineViewMode,

    pub arcade: ArcadeView,
    pub dashboard: CreatorDashboardView,
}

#[derive(Default)]
pub struct ArcadeView {
    pub front_page: Option<FrontPageResponse>,
    tab: ArcadeTab,
}

#[derive(Default, PartialEq)]
enum ArcadeTab {
    #[default]
    Popular,
    TopRated,
    New,
}

impl ArcadeView {
    fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;
        if let Some(front_page) = &self.front_page {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, ArcadeTab::Popular, "Popular Games");
                ui.selectable_value(&mut self.tab, ArcadeTab::TopRated, "Top Rated");
                ui.selectable_value(&mut self.tab, ArcadeTab::New, "New Releases");
            });

            let slice = match self.tab {
                ArcadeTab::Popular => &front_page.popular_games_ids,
                ArcadeTab::TopRated => &front_page.top_rated_games_ids,
                ArcadeTab::New => &front_page.new_games_ids,
            };

            // TODO: Add space for image
            egui::Grid::new("arcade_grid")
                .num_columns(7)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    let mut num = 1;
                    ui.label("#");
                    ui.label("Title");
                    ui.label("Short Description");
                    ui.label("Rating");
                    ui.label("Tags");
                    ui.label("Size");
                    ui.label("Download");
                    ui.end_row();

                    for id in slice.iter() {
                        // TOOD: Optimize this
                        if let Some(game) = front_page.games.iter().find(|game| game.game_id == *id)
                        {
                            ui.label(format!("{num}"));
                            num += 1;

                            ui.label(&game.title);
                            ui.label(&game.short_description);
                            ui.label(format!("{}", game.average_rating));
                            ui.label("TODO: Tags");
                            ui.label(format!(
                                "{}.mb",
                                game.rom_size.unwrap_or_default() as f32 / (1024.0 * 1024.0)
                            ));

                            // TODO: Add a list of pending downloads so we dont spam the server
                            if ui.button("Download").clicked() {
                                context.task_manager.http.try_download_rom(
                                    game.game_id,
                                    &context.auth_state.get_session().unwrap(),
                                )
                            }
                        }
                    }
                });
        } else {
            ui.spinner();
        };
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
