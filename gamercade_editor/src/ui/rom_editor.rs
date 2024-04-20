use std::fs;

use eframe::egui::{self, Button, Ui};
use gamercade_core::{FrameRate, ResolutionRatio, ResolutionSize};
use gamercade_fs::{DataPack, EditorRom};

#[derive(Debug, Clone, Default)]
pub struct RomEditor {}

impl RomEditor {
    pub fn draw_contents(&self, ui: &mut Ui, rom: &mut EditorRom) {
        ui.group(|ui| {
            ui.label(format!(
                "Resolution: {} x {} {}",
                rom.resolution.width(),
                rom.resolution.height(),
                rom.resolution.ratio.as_str(),
            ));
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut rom.resolution.ratio,
                    ResolutionRatio::Standard,
                    ResolutionRatio::Standard.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.ratio,
                    ResolutionRatio::Square,
                    ResolutionRatio::Square.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.ratio,
                    ResolutionRatio::Classic,
                    ResolutionRatio::Classic.as_str(),
                );
            });
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::UltraLow,
                    ResolutionSize::UltraLow.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::VeryLow,
                    ResolutionSize::VeryLow.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::Low,
                    ResolutionSize::Low.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::Medium,
                    ResolutionSize::Medium.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::High,
                    ResolutionSize::High.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::VeryHigh,
                    ResolutionSize::VeryHigh.as_str(),
                );
                ui.selectable_value(
                    &mut rom.resolution.size,
                    ResolutionSize::UltraHigh,
                    ResolutionSize::UltraHigh.as_str(),
                );
            });
        });

        ui.group(|ui| {
            ui.label(format!(
                "Frame Rate: {}fps",
                rom.frame_rate.frames_per_second()
            ));
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::UltraSlow,
                    FrameRate::UltraSlow.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::VerySlow,
                    FrameRate::VerySlow.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::Slow,
                    FrameRate::Slow.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::Moderate,
                    FrameRate::Moderate.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::Normal,
                    FrameRate::Normal.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::Fast,
                    FrameRate::Fast.as_str(),
                );
                ui.selectable_value(
                    &mut rom.frame_rate,
                    FrameRate::UltraFast,
                    FrameRate::UltraFast.as_str(),
                );
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

        ui.group(|ui| {
            let exists = rom.data_pack.is_some();
            let text = if exists { "Exists" } else { "None" };
            ui.label(format!("Data Pack: {text}"));

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(!exists, Button::new("Load Data Pack"))
                    .clicked()
                {
                    match try_load_data_pack() {
                        Ok(data_pack) => rom.data_pack = Some(data_pack),
                        Err(e) => println!("{e}"),
                    }
                }

                if ui
                    .add_enabled(exists, Button::new("Remove Data Pack"))
                    .clicked()
                {
                    rom.data_pack = None
                }
            });
        });
    }
}

fn try_load_data_pack() -> Result<DataPack, String> {
    let path = match rfd::FileDialog::new()
        .set_title("Load Data Pack...")
        .set_directory("/")
        .pick_file()
    {
        Some(path) => path,
        None => return Err("No file selected".to_string()),
    };

    let data = fs::read(path).map_err(|e| e.to_string())?;

    Ok(DataPack { data })
}
