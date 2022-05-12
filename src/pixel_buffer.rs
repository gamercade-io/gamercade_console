use std::ops::{Index, IndexMut, Range};

use crate::{Palette, Rom, SpriteIndex, SpriteSheet, BYTES_PER_PIXEL};

#[derive(Clone)]
pub struct PixelBuffer {
    pub pixel_buffer: Box<[u8]>,
    pub buffer_width: usize,
    pub buffer_height: usize,
}

impl PixelBuffer {
    pub fn init_from_rom(rom: &Rom) -> Self {
        let pixel_buffer = (0..rom.resolution.total_pixels() * BYTES_PER_PIXEL as i32)
            .map(|_| 0)
            .collect::<Vec<u8>>()
            .into_boxed_slice();

        Self {
            pixel_buffer,
            buffer_width: rom.resolution.width() as usize,
            buffer_height: rom.resolution.height() as usize,
        }
    }

    pub fn draw_sprite(
        &mut self,
        sheet: &SpriteSheet,
        sprite_index: SpriteIndex,
        palette: &Palette,
        x: i32,
        y: i32,
    ) {
        let palette = palette.as_pixel_colors();
        let sprite_width = sheet.width;
        let sprite_height = sheet.height;
        let sprite = &sheet[sprite_index];

        let start = (y * self.buffer_width as i32) + x;

        let sprite_start_x = x.min(0).unsigned_abs() as usize;
        let sprite_start_y = y.min(0).unsigned_abs() as usize;
        let sprite_bounds_width = (self.buffer_width as i32 - x)
            .min(sprite_width as i32)
            .max(0) as usize;
        let sprite_bounds_height = (self.buffer_height as i32 - y)
            .min(sprite_height as i32)
            .max(0) as usize;

        (sprite_start_y..sprite_bounds_height).for_each(|y| {
            (sprite_start_x..sprite_bounds_width).for_each(|x| {
                let target_pixel = start + x as i32 + (y as i32 * self.buffer_width as i32);
                let target_pixel = target_pixel as usize * BYTES_PER_PIXEL;

                let color_index = sprite.data[x + (y * sprite_width)];
                let color = palette[color_index.0];
                self.pixel_buffer[target_pixel..target_pixel + BYTES_PER_PIXEL]
                    .copy_from_slice(&color);
            });
        });
    }
}

impl Index<usize> for PixelBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixel_buffer[index]
    }
}

impl Index<Range<usize>> for PixelBuffer {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.pixel_buffer[index.start..index.end]
    }
}

impl IndexMut<usize> for PixelBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixel_buffer[index]
    }
}

impl IndexMut<Range<usize>> for PixelBuffer {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.pixel_buffer[index.start..index.end]
    }
}
