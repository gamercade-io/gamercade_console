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

    fn line(
        &self,
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
        color_index: Option<usize>,
        palette_index: Option<usize>,
    ) {
        // Optimized horizontal or veritcal lines
        if x0 == x1 {
            self.draw_line_vertical(x0, y0, y1, color_index, palette_index);
            return;
        } else if y0 == y1 {
            self.draw_line_horizontal(x0, x1, y0, color_index, palette_index);
            return;
        }

        let x0 = x0 as i32;
        let x1 = x1 as i32;
        let y0 = y0 as i32;
        let y1 = y1 as i32;

        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low(x1, y1, x0, y0, color_index, palette_index)
            } else {
                self.draw_line_low(x0, y0, x1, y1, color_index, palette_index)
            }
        } else {
            if y0 > y1 {
                self.draw_line_high(x1, y1, x0, y0, color_index, palette_index)
            } else {
                self.draw_line_high(x0, y0, x1, y1, color_index, palette_index)
            }
        }
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

    fn draw_line_low(
        &self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color_index: Option<usize>,
        palette_index: Option<usize>,
    ) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;

        let y_adjust = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };

        let mut d = (2 * dy) - dx;
        let mut y = y0;

        (x0..x1).for_each(|x| {
            self.set_pixel(x as u32, y as u32, color_index, palette_index);
            if d > 0 {
                y = y + y_adjust;
                d = d + (2 * (dy - dx));
            } else {
                d = d + 2 * dy;
            }
        })
    }

    fn draw_line_high(
        &self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color_index: Option<usize>,
        palette_index: Option<usize>,
    ) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;

        let x_adjust = if dx < 0 {
            dx = -dx;
            -1
        } else {
            1
        };

        let mut d = (2 * dx) - dy;
        let mut x = x0;

        (y0..y1).for_each(|y| {
            self.set_pixel(x as u32, y as u32, color_index, palette_index);
            if d > 0 {
                x = x + x_adjust;
                d = d + (2 * (dx - dy));
            } else {
                d = d + 2 * dx;
            }
        })
    }

    fn draw_line_vertical(
        &self,
        x: u32,
        y0: u32,
        y1: u32,
        color_index: Option<usize>,
        palette_index: Option<usize>,
    ) {
        let (start, end) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

        (start..end).for_each(|y| {
            self.set_pixel(x, y, color_index, palette_index);
        });
    }

    fn draw_line_horizontal(
        &self,
        x0: u32,
        x1: u32,
        y: u32,
        color_index: Option<usize>,
        palette_index: Option<usize>,
    ) {
        let (start, end) = if x0 < x1 { (x0, x1) } else { (x1, x0) };

        (start..end).for_each(|x| {
            self.set_pixel(x, y, color_index, palette_index);
        });
    }
}
