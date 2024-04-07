use std::fs;

use eframe::egui::{self, Button, Ui};
use gamercade_core::{
    FrameRate,
    Resolution::{High, Low, Medium, UltraHigh, UltraLow, VeryHigh, VeryLow},
};
use gamercade_fs::{DataPack, EditorRom};

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
                ui.selectable_value(&mut rom.resolution, UltraLow, "Ultra Low");
                ui.selectable_value(&mut rom.resolution, VeryLow, "Very Low");
                ui.selectable_value(&mut rom.resolution, Low, "Low");
                ui.selectable_value(&mut rom.resolution, Medium, "Medium");
                ui.selectable_value(&mut rom.resolution, High, "High");
                ui.selectable_value(&mut rom.resolution, VeryHigh, "Very High");
                ui.selectable_value(&mut rom.resolution, UltraHigh, "Ultra High");
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
