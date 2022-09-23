use std::{path::PathBuf};

use gamercade_core::{Color, Palette};
use gamercade_fs::{EditorPalette, EditorRom};

fn main() {
    let step_rg = 256_f32 / 31.0;
    let step_b = 256_f32 / 15.0;
    let mut all_colors = Vec::with_capacity(32 * 32 * 16);

    (0..32).for_each(|r| {
        (0..32).for_each(|g| {
            (0..16).for_each(|b| {
                all_colors.push(Color {
                    r: (r as f32 * step_rg).round().clamp(0.0, 255.0) as u8,
                    g: (g as f32 * step_rg).round().clamp(0.0, 255.0) as u8,
                    b: (b as f32 * step_b).round().clamp(0.0, 255.0) as u8,
                    a: 0xff,
                });
            })
        })
    });

    let all_palettes = all_colors
        .chunks_exact(64)
        .enumerate()
        .map(|(index, colors)| EditorPalette {
            name: index.to_string(),
            palette: Palette {
                colors: colors.to_owned().try_into().unwrap(),
            },
        })
        .collect::<Vec<_>>();

    let mut rom = EditorRom::default();

    rom.graphics.palettes = all_palettes;

    let mut path = PathBuf::new();
    path.set_file_name("5r5g4b_color.gce");

    rom.try_save(&path).unwrap();
}
