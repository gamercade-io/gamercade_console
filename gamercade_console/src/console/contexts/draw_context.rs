use crate::{api::DrawApi, pixel_buffer::PixelBuffer};
use gamercade_core::{Color, GraphicsParameters, XCord, YCord, BYTES_PER_PIXEL};
use gamercade_fs::Rom;
use std::{
    ops::{Add, Sub},
    sync::Arc,
};

#[derive(Clone)]
pub struct DrawContext {
    pub(crate) frame_buffer: PixelBuffer,
    pub(crate) rom: Arc<Rom>,
}

impl DrawContext {
    pub fn new(rom: Arc<Rom>) -> Self {
        Self {
            frame_buffer: PixelBuffer::init_from_rom(&rom),
            rom,
        }
    }

    pub fn try_get_xcord<T: Into<i32>>(&self, x: T) -> Option<XCord> {
        self.rom.resolution.try_get_xcord(x)
    }

    pub fn try_get_ycord<T: Into<i32>>(&self, y: T) -> Option<YCord> {
        self.rom.resolution.try_get_ycord(y)
    }
}

impl DrawApi for DrawContext {
    fn sprite(&mut self, graphics_parameters: i32, transparency_mask: i64, x: i32, y: i32) {
        let GraphicsParameters {
            palette_index,
            sprite_sheet_index,
            sprite_index,
            flip_x,
            flip_y,
            ..
        } = graphics_parameters.into();

        let palette = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette,
            None => return,
        };
        let sheet = match self.rom.graphics.sprite_sheet(sprite_sheet_index) {
            Some(sheet) => sheet,
            None => return,
        };

        self.frame_buffer.draw_sprite(
            sheet,
            sprite_index,
            palette,
            (x, y),
            transparency_mask,
            (flip_x, flip_y),
        );
    }

    fn clear_screen(&mut self, graphics_parameters: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        self.frame_buffer
            .clear_buffer(color_index, palette_index, &self.rom);
    }

    fn set_pixel(&mut self, graphics_parameters: i32, x: i32, y: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        if let (Some(x), Some(y)) = (self.try_get_xcord(x), self.try_get_ycord(y)) {
            if let Some(palette) = self.rom.graphics.palette(palette_index) {
                let color = palette[color_index];
                self.set_pixel_safe(x, y, color)
            }
        }
    }

    fn line(&mut self, graphics_parameters: i32, x0: i32, y0: i32, x1: i32, y1: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        let palette = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette,
            None => return,
        };

        let color = palette[color_index];

        // Optimized horizontal or veritcal lines
        if x0 == x1 {
            self.draw_line_vertical(x0, y0, y1, color);
            return;
        } else if y0 == y1 {
            self.draw_line_horizontal(x0, x1, y0, color);
            return;
        }

        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low(x1, y1, x0, y0, color);
            } else {
                self.draw_line_low(x0, y0, x1, y1, color);
            }
        } else if y0 > y1 {
            self.draw_line_high(x1, y1, x0, y0, color);
        } else {
            self.draw_line_high(x0, y0, x1, y1, color);
        }
    }

    fn rect(&mut self, graphics_parameters: i32, x: i32, y: i32, width: i32, height: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        let color = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette[color_index],
            None => return,
        };

        let x1 = x + width;
        let y1 = y + height;

        // Top
        self.draw_line_horizontal(x, x1, y, color);

        // Bottom
        self.draw_line_horizontal(x, x1, y1, color);

        // Left
        self.draw_line_vertical(x, y, y1, color);

        // Right
        self.draw_line_vertical(x1, y, y1, color);
    }

    fn rect_filled(&mut self, graphics_parameters: i32, x: i32, y: i32, width: i32, height: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        let color = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette[color_index],
            None => return,
        };

        let x1 = x + width;
        let y1 = y + height;

        (y..y1).for_each(|y| {
            self.draw_line_horizontal(x, x1, y, color);
        })
    }

    fn circle(&mut self, graphics_parameters: i32, x: i32, y: i32, radius: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        let color = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette[color_index],
            None => return,
        };

        let mut f = 1 - radius;
        let mut ddf_x = 0;
        let mut ddf_y = -2 * radius;

        let x0 = x;
        let y0 = y;
        let mut x = 0;
        let mut y = radius;

        self.draw_circle_points(x0, y0, x, y, color);
        while x < y {
            if f >= 0 {
                y -= 1;
                ddf_y += 2;
                f += ddf_y;
            };

            x += 1;
            ddf_x += 2;
            f += ddf_x + 1;
            self.draw_circle_points(x0, y0, x, y, color);
        }
    }

    fn circle_filled(&mut self, graphics_parameters: i32, x: i32, y: i32, radius: i32) {
        let GraphicsParameters {
            color_index,
            palette_index,
            ..
        } = graphics_parameters.into();

        let color = match self.rom.graphics.palette(palette_index) {
            Some(palette) => palette[color_index],
            None => return,
        };

        let mut f = 1 - radius;
        let mut ddf_x = 0;
        let mut ddf_y = -2 * radius;

        let x0 = x;
        let y0 = y;
        let mut x = 0;
        let mut y = radius;

        self.draw_line_horizontal(x0 - radius, x0 + radius, y0, color);
        while x < y {
            if f >= 0 {
                y -= 1;
                ddf_y += 2;
                f += ddf_y;
            };

            x += 1;
            ddf_x += 2;
            f += ddf_x + 1;
            self.draw_line_horizontal(x0 - x, x0 + x, y0 + y, color);
            self.draw_line_horizontal(x0 - y, x0 + y, y0 + x, color);
            self.draw_line_horizontal(x0 - x, x0 + x, y0 - y, color);
            self.draw_line_horizontal(x0 - y, x0 + y, y0 - x, color);
        }
    }

    fn write_pixel_buffer(&mut self, start_index: usize, data: &[u32]) {
        (start_index
            ..data
                .len()
                .min(self.frame_buffer.pixel_buffer.len() / BYTES_PER_PIXEL))
            .zip(data.iter())
            .for_each(|(index, gp)| {
                let GraphicsParameters {
                    color_index,
                    palette_index,
                    ..
                } = GraphicsParameters::from(*gp);

                let color = match self.rom.graphics.palette(palette_index) {
                    Some(palette) => palette[color_index],
                    None => return,
                };

                self.frame_buffer.pixel_buffer
                    [index * BYTES_PER_PIXEL..(index + 1) * BYTES_PER_PIXEL]
                    .copy_from_slice(&color.into_pixel_data());
            });
    }
}

impl DrawContext {
    fn width(&self) -> i32 {
        self.rom.width()
    }

    fn height(&self) -> i32 {
        self.rom.height()
    }

    fn set_pixel_safe(&mut self, x: XCord, y: YCord, color: Color) {
        let pixel_index = self.x_y_cord_to_pixel_buffer_index(x, y);
        let color = color.into_pixel_data();
        if let Some(index_bound) = pixel_index.checked_add(BYTES_PER_PIXEL) {
            if let Some(pixel_buffer) = self
                .frame_buffer
                .pixel_buffer
                .get_mut(pixel_index..index_bound)
            {
                pixel_buffer.copy_from_slice(&color);
            }
        }
    }

    fn try_set_pixel_safe(&mut self, x: Option<XCord>, y: Option<YCord>, color: Color) {
        if let (Some(x), Some(y)) = (x, y) {
            self.set_pixel_safe(x, y, color)
        }
    }

    fn x_y_cord_to_pixel_buffer_index(&self, x: XCord, y: YCord) -> usize {
        (x.raw_value() + (y.raw_value() * self.width() as usize)) * BYTES_PER_PIXEL
    }

    fn draw_line_low(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;

        let rem_limit = (dx + 1) / 2;
        let mut x = x0.max(0);
        let xe = x1.min(self.width() - 1);

        let mut rem = ((x - x0) * dy % dx) - rem_limit;
        let mut y = y0 + (x - x0) * dy / dx;

        let y_adjust = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };

        while x <= xe {
            if let (Some(valid_x), Some(valid_y)) = (self.try_get_xcord(x), self.try_get_ycord(y)) {
                self.set_pixel_safe(valid_x, valid_y, color);
            }
            x += 1;
            rem += dy;
            if rem >= 0 {
                rem -= dx;
                y += y_adjust;
            }
        }
    }

    fn draw_line_high(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;

        let rem_limit = (dy + 1) / 2;
        let mut y = y0.max(0);
        let ye = y1.min(self.height() - 1);

        let mut rem = ((y - y0) * dx % dy) - rem_limit;
        let mut x = x0 + (y - y0) * dx / dy;

        let x_adjust = if dx < 0 {
            dx = -dx;
            -1
        } else {
            1
        };

        while y <= ye {
            if let (Some(valid_x), Some(valid_y)) = (self.try_get_xcord(x), self.try_get_ycord(y)) {
                self.set_pixel_safe(valid_x, valid_y, color);
            }
            y += 1;
            rem += dx;
            if rem >= 0 {
                rem -= dy;
                x += x_adjust;
            }
        }
    }

    // TODO: Can optimize this further with direct access into
    // the pixel buffers?
    fn draw_line_vertical(&mut self, x: i32, y0: i32, y1: i32, color: Color) {
        if x < 0 || x > self.width() - 1 {
            return;
        }

        let (start, end) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

        if start > self.height() || end < 0 {
            return;
        }

        let start = start.max(0) as usize;
        let end = end.min(self.height()) as usize;

        let width = self.width() as usize;
        let start_index = (start * width) + x as usize;
        let pixel_count = (end - start) + 1;
        let color = color.into_pixel_data();

        self.frame_buffer
            .pixel_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .skip(start_index)
            .step_by(width)
            .take(pixel_count)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    /// Efficiently draws a horizontal line with direct array access
    fn draw_line_horizontal(&mut self, x0: i32, x1: i32, y: i32, color: Color) {
        if y < 0 || y > self.height() {
            return;
        }

        let (start, end) = if x0 < x1 { (x0, x1) } else { (x1, x0) };

        if start > self.width() - 1 || end < 0 {
            return;
        }

        let start = start.max(0) as usize;
        let end = end.min(self.width() - 1) as usize;
        let y = y as usize;

        let start_index = (y * self.width() as usize) + start;
        let pixel_count = (end - start) + 1;
        let color = color.into_pixel_data();

        self.frame_buffer
            .pixel_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .skip(start_index)
            .take(pixel_count)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    /// Draws the 8 circle points
    fn draw_circle_points(&mut self, x0: i32, y0: i32, x: i32, y: i32, color: Color) {
        let up_x = self.try_get_ycord(y0.add(x));
        let up_y = self.try_get_ycord(y0.add(y));
        let down_x = self.try_get_ycord(y0.sub(x));
        let down_y = self.try_get_ycord(y0.sub(y));
        let left_x = self.try_get_xcord(x0.sub(x));
        let left_y = self.try_get_xcord(x0.sub(y));
        let right_x = self.try_get_xcord(x0.add(x));
        let right_y = self.try_get_xcord(x0.add(y));

        self.try_set_pixel_safe(right_x, up_y, color);
        self.try_set_pixel_safe(right_x, down_y, color);
        self.try_set_pixel_safe(right_y, up_x, color);
        self.try_set_pixel_safe(right_y, down_x, color);
        self.try_set_pixel_safe(left_y, up_x, color);
        self.try_set_pixel_safe(left_y, down_x, color);
        self.try_set_pixel_safe(left_x, up_y, color);
        self.try_set_pixel_safe(left_x, down_y, color);
    }
}
