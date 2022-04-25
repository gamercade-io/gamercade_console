use crate::{ColorIndex, PaletteIndex, SpriteIndex, SpriteSheetIndex, BYTES_PER_PIXEL};

use super::graphics::Resolution;
use serde::{Deserialize, Serialize};

use super::{
    graphics::{FrameRate, GraphicsData},
    SoundsData,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub graphics: GraphicsData,
    pub sounds: SoundsData,
    //pub code: Box<[u8]>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Resolution::Low,
            frame_rate: FrameRate::Fast,
            graphics: GraphicsData::default(),
            sounds: SoundsData::default(),
        }
    }
}

impl Rom {
    pub fn clear_buffer(&self, color: ColorIndex, palette: PaletteIndex, target: &mut [u8]) {
        let color = self.graphics.palette(palette)[color].into_pixel_data();
        target
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    pub fn height(&self) -> i32 {
        self.resolution.height()
    }

    pub fn width(&self) -> i32 {
        self.resolution.width()
    }

    pub fn draw_sprite(
        &self,
        sheet: SpriteSheetIndex,
        sprite: SpriteIndex,
        palette: PaletteIndex,
        (x, y): (usize, usize),
        buffer_width: usize,
        target: &mut [u8],
    ) {
        let palette = self.graphics.palette(palette).as_pixel_colors();
        let sheet = self.graphics.sprite_sheet(sheet);
        let sprite_width = sheet.width;
        let sprite_height = sheet.height;
        let sprite = &sheet[sprite];

        let start = (y * buffer_width) + x;

        (0..sprite_height).for_each(|y| {
            (0..sprite_width).for_each(|x| {
                let target_pixel = (start + x + (y * buffer_width)) * BYTES_PER_PIXEL;
                let color_index = sprite.data[x + (y * sprite_width)];
                let color = palette[color_index.0];
                target[target_pixel..target_pixel + BYTES_PER_PIXEL].copy_from_slice(&color);
            });
        });
    }
}
