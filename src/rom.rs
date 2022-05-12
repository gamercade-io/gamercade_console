use crate::{ColorIndex, PaletteIndex, PixelBuffer, BYTES_PER_PIXEL};

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
    pub fn clear_buffer(&self, color: ColorIndex, palette: PaletteIndex, target: &mut PixelBuffer) {
        let color = self.graphics.palette(palette)[color].into_pixel_data();
        target
            .pixel_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    pub fn height(&self) -> i32 {
        self.resolution.height()
    }

    pub fn width(&self) -> i32 {
        self.resolution.width()
    }
}
