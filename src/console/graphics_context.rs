use parking_lot::Mutex;
use std::sync::Arc;

use crate::{api::GraphicsApi, core::Rom};

#[derive(Clone)]
pub struct GraphicsContext {
    pub frame_buffer: Arc<Mutex<Box<[u8]>>>,
    pub rom: Arc<Rom>,
}

impl GraphicsApi for GraphicsContext {
    fn clear_screen(&self, color_index: Option<usize>, palette_index: Option<usize>) {
        let color = self.get_color_as_pixel_data(
            color_index.unwrap_or_default(),
            palette_index.unwrap_or_default(),
        );

        self.frame_buffer
            .lock()
            .chunks_exact_mut(4)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    fn set_pixel(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>) {
        let pixel_index = self.x_y_to_pixel(x, y);
        let color = self.get_color_as_pixel_data(
            color_index.unwrap_or_default(),
            palette_index.unwrap_or_default(),
        );

        self.frame_buffer.lock()[pixel_index..pixel_index + 4].copy_from_slice(&color);
    }

    fn height(&self) -> u32 {
        self.rom.resolution.height()
    }

    fn width(&self) -> u32 {
        self.rom.resolution.width()
    }
}

impl GraphicsContext {
    fn x_y_to_pixel(&self, x: u32, y: u32) -> usize {
        ((x + (y * self.rom.resolution.width())) * 4) as usize
    }

    fn get_color_as_pixel_data(&self, color_index: usize, palette_index: usize) -> [u8; 4] {
        let color = self.rom.palettes[palette_index].colors[color_index];
        [color.r, color.g, color.b, 0xff]
    }
}
