use eframe::egui::{self, Ui};
use gamercade_core::{FrameRate, Resolution};

use crate::editor_data::EditorRom;

#[derive(Debug, Clone, Default)]
pub struct RomEditor {}

impl RomEditor {
    pub fn draw_contents(&self, ui: &mut Ui, rom: &mut EditorRom) {
        ui.group(|ui| {
            ui.label(format!(
                "Resolution: {} x {}",
                rom.resolution.width(),
                rom.resolution.height()
            ));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut rom.resolution, Resolution::UltraLow, "Ultra Low");
                ui.selectable_value(&mut rom.resolution, Resolution::VeryLow, "Very Low");
                ui.selectable_value(&mut rom.resolution, Resolution::Low, "Low");
                ui.selectable_value(&mut rom.resolution, Resolution::Medium, "Medium");
                ui.selectable_value(&mut rom.resolution, Resolution::High, "High");
                ui.selectable_value(&mut rom.resolution, Resolution::VeryHigh, "Very High");
                ui.selectable_value(&mut rom.resolution, Resolution::UltraHigh, "Ultra High");
            });
        });

        ui.group(|ui| {
            ui.label(format!(
                "Frame Rate: {}fps",
                rom.frame_rate.frames_per_second()
            ));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut rom.frame_rate, FrameRate::SuperSlow, "Super Slow");
                ui.selectable_value(&mut rom.frame_rate, FrameRate::Slow, "Slow");
                ui.selectable_value(&mut rom.frame_rate, FrameRate::Normal, "Normal");
                ui.selectable_value(&mut rom.frame_rate, FrameRate::Fast, "Fast");
                ui.selectable_value(&mut rom.frame_rate, FrameRate::SuperFast, "Super Fast");
            });
        });

        ui.group(|ui| {
            ui.label(format!(
                "Player Count: {} - {} player(s)",
                rom.player_count.0, rom.player_count.1
            ));
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut rom.player_count.0, 1..=(4.min(rom.player_count.1)))
                        .text("Min"),
                );
                ui.add(
                    egui::Slider::new(&mut rom.player_count.1, (1.max(rom.player_count.0))..=4)
                        .text("Max"),
                );
            });
        });
    }
}
