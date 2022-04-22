use std::sync::Arc;

use crate::{
    api::GraphicsApi,
    core::{ColorIndex, PaletteIndex, Rom},
    BYTES_PER_PIXEL,
};

#[derive(Clone)]
pub struct GraphicsContext {
    pub(crate) frame_buffer: Box<[u8]>,
    pub(crate) rom: Arc<Rom>,
}

impl GraphicsContext {
    pub fn new(rom: Arc<Rom>) -> Self {
        Self {
            frame_buffer: init_frame_buffer(&rom),
            rom,
        }
    }
}

fn init_frame_buffer(rom: &Rom) -> Box<[u8]> {
    (0..rom.resolution.total_pixels() * BYTES_PER_PIXEL as i32)
        .map(|_| 0)
        .collect::<Vec<u8>>()
        .into_boxed_slice()
}

impl GraphicsApi for GraphicsContext {
    fn clear_screen(&mut self, color_index: i32, palette_index: i32) {
        if let (Ok(color_index), Ok(palette_index)) =
            (color_index.try_into(), self.validate_palette(palette_index))
        {
            let color = self.get_color_as_pixel_data(color_index, palette_index);

            self.frame_buffer
                .chunks_exact_mut(BYTES_PER_PIXEL)
                .for_each(|pixel| pixel.copy_from_slice(&color));
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color_index: i32, palette_index: i32) {
        if let (Ok(x), Ok(y), Ok(color_index), Ok(palette_index)) = (
            self.validate_x(x),
            self.validate_y(y),
            color_index.try_into(),
            self.validate_palette(palette_index),
        ) {
            self.set_pixel_safe(x, y, color_index, palette_index);
        }
    }

    fn height(&self) -> i32 {
        self.rom.resolution.height()
    }

    fn width(&self) -> i32 {
        self.rom.resolution.width()
    }

    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color_index: i32, palette_index: i32) {
        let x0 = self.validate_x(x0);
        let y0 = self.validate_y(y0);
        let x1 = self.validate_x(x1);
        let y1 = self.validate_y(y1);
        let color_index = color_index.try_into();
        let palette_index = self.validate_palette(palette_index);

        if x0.is_err()
            || y0.is_err()
            || x1.is_err()
            || y1.is_err()
            || color_index.is_err()
            || palette_index.is_err()
        {
            return;
        }

        let x0 = x0.unwrap();
        let y0 = y0.unwrap();
        let x1 = x1.unwrap();
        let y1 = y1.unwrap();
        let color_index = color_index.unwrap();
        let palette_index = palette_index.unwrap();

        // Optimized horizontal or veritcal lines
        if x0 == x1 {
            self.draw_line_vertical(x0, y0, y1, color_index, palette_index);
            return;
        } else if y0 == y1 {
            self.draw_line_horizontal(x0, x1, y0, color_index, palette_index);
            return;
        }

        let x0 = x0.0 as i32;
        let y0 = y0.0 as i32;
        let x1 = x1.0 as i32;
        let y1 = y1.0 as i32;

        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low(x1, y1, x0, y0, color_index, palette_index)
            } else {
                self.draw_line_low(x0, y0, x1, y1, color_index, palette_index)
            }
        } else if y0 > y1 {
            self.draw_line_high(x1, y1, x0, y0, color_index, palette_index)
        } else {
            self.draw_line_high(x0, y0, x1, y1, color_index, palette_index)
        }
    }

    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    ) {
        let x1 = self.validate_x(x + width);
        let y1 = self.validate_y(y + height);
        let x = self.validate_x(x);
        let y = self.validate_y(y);
        let color_index = color_index.try_into();
        let palette_index = self.validate_palette(palette_index);

        if x1.is_err()
            || y1.is_err()
            || x.is_err()
            || y.is_err()
            || color_index.is_err()
            || palette_index.is_err()
        {
            return;
        };

        let x1 = x1.unwrap();
        let y1 = y1.unwrap();
        let x = x.unwrap();
        let y = y.unwrap();
        let color_index = color_index.unwrap();
        let palette_index = palette_index.unwrap();

        // Top
        self.draw_line_horizontal(x, x1, y, color_index, palette_index);

        // Bottom
        self.draw_line_horizontal(x, x1, y1, color_index, palette_index);

        // Left
        self.draw_line_vertical(x, y, y1, color_index, palette_index);

        // Right
        self.draw_line_vertical(x1, y, y1, color_index, palette_index);
    }

    fn rect_filled(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    ) {
        let x1 = self.validate_x(x + width);
        let y1 = self.validate_y(y + height);
        let x = self.validate_x(x);
        let y = self.validate_y(y);
        let color_index = color_index.try_into();
        let palette_index = self.validate_palette(palette_index);

        if x1.is_err()
            || y1.is_err()
            || x.is_err()
            || y.is_err()
            || color_index.is_err()
            || palette_index.is_err()
        {
            return;
        };

        let x1 = x1.unwrap();
        let y1 = y1.unwrap();
        let x = x.unwrap();
        let y = y.unwrap();
        let color_index = color_index.unwrap();
        let palette_index = palette_index.unwrap();

        (y.0..y1.0).for_each(|y| {
            self.draw_line_horizontal(x, x1, YCord(y), color_index, palette_index);
        })
    }

    fn circle(&mut self, x: i32, y: i32, radius: i32, color_index: i32, palette_index: i32) {
        let top = self.validate_y(y - radius);
        let bottom = self.validate_y(y + radius);
        let left = self.validate_x(x - radius);
        let right = self.validate_x(x + radius);
        let color_index = color_index.try_into();
        let palette_index = self.validate_palette(palette_index);

        if top.is_err()
            || bottom.is_err()
            || left.is_err()
            || right.is_err()
            || color_index.is_err()
            || palette_index.is_err()
        {
            return;
        }

        let x0 = x as usize;
        let y0 = y as usize;

        let mut f = 1 - radius;
        let mut ddf_x = 0;
        let mut ddf_y = -2 * radius;
        let mut x = 0;
        let mut y = radius;

        let color_index = color_index.unwrap();
        let palette_index = palette_index.unwrap();

        self.set_pixel_safe(
            XCord(x0),
            YCord(y0 + radius as usize),
            color_index,
            palette_index,
        );
        self.set_pixel_safe(
            XCord(x0),
            YCord(y0 - radius as usize),
            color_index,
            palette_index,
        );
        self.set_pixel_safe(
            XCord(x0 + radius as usize),
            YCord(y0 as usize),
            color_index,
            palette_index,
        );
        self.set_pixel_safe(
            XCord(x0 - radius as usize),
            YCord(y0 as usize),
            color_index,
            palette_index,
        );

        while x < y {
            if f >= 0 {
                y -= 1;
                ddf_y += 2;
                f += ddf_y;
            };

            x += 1;
            ddf_x += 2;
            f += ddf_x + 1;
            self.set_pixel_safe(
                XCord(x0 + x as usize),
                YCord(y0 + y as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 - x as usize),
                YCord(y0 + y as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 + x as usize),
                YCord(y0 - y as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 - x as usize),
                YCord(y0 - y as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 + y as usize),
                YCord(y0 + x as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 - y as usize),
                YCord(y0 + x as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 + y as usize),
                YCord(y0 - x as usize),
                color_index,
                palette_index,
            );
            self.set_pixel_safe(
                XCord(x0 - y as usize),
                YCord(y0 - x as usize),
                color_index,
                palette_index,
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct XCord(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct YCord(usize);

impl GraphicsContext {
    fn validate_palette(&self, index: i32) -> Result<PaletteIndex, &'static str> {
        self.rom.graphics.validate_palette_index(index)
    }

    fn validate_x(&self, x: i32) -> Result<XCord, &'static str> {
        if x >= 0 && x < self.width() {
            Ok(XCord(x as usize))
        } else {
            Err("invalid X screen coordinate")
        }
    }

    fn validate_y(&self, y: i32) -> Result<YCord, &'static str> {
        if y >= 0 && y < self.height() {
            Ok(YCord(y as usize))
        } else {
            Err("invalid Y screen coordinate")
        }
    }

    fn set_pixel_safe(
        &mut self,
        x: XCord,
        y: YCord,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
    ) {
        let pixel_index = self.x_y_cord_to_pixel_buffer_index(x, y);
        let color = self.get_color_as_pixel_data(color_index, palette_index);
        self.frame_buffer[pixel_index..pixel_index + BYTES_PER_PIXEL].copy_from_slice(&color);
    }

    fn x_y_cord_to_pixel_buffer_index(&self, x: XCord, y: YCord) -> usize {
        (x.0 + (y.0 * self.width() as usize)) * BYTES_PER_PIXEL
    }

    // TODO: Is this needed?
    // fn x_y_to_pixel_buffer_index(&self, x: i32, y: i32) -> Option<usize> {
    //     let width = self.width();
    //     let height = self.height();
    //     let index = (x + (y * width)) * BYTES_PER_PIXEL as i32;

    //     if index < (width * height * BYTES_PER_PIXEL as i32) {
    //         Some(index as usize)
    //     } else {
    //         None
    //     }
    // }

    fn get_color_as_pixel_data(
        &self,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
    ) -> [u8; BYTES_PER_PIXEL] {
        let color = self.rom.graphics.palette(palette_index).color(color_index);
        [color.r, color.g, color.b, 0xff]
    }

    fn draw_line_low(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
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

        for x in x0..=x1 {
            if let (Ok(valid_x), Ok(valid_y)) = (self.validate_x(x), self.validate_y(y)) {
                self.set_pixel_safe(valid_x, valid_y, color_index, palette_index);
                if d > 0 {
                    y += y_adjust;
                    d += 2 * (dy - dx);
                } else {
                    d += 2 * dy;
                }
            } else {
                return;
            }
        }
    }

    fn draw_line_high(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
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

        for y in y0..=y1 {
            if let (Ok(valid_x), Ok(valid_y)) = (self.validate_x(x), self.validate_y(y)) {
                self.set_pixel_safe(valid_x, valid_y, color_index, palette_index);
                if d > 0 {
                    x += x_adjust;
                    d += 2 * (dx - dy);
                } else {
                    d += 2 * dx;
                }
            } else {
                return;
            }
        }
    }

    // TODO: Can optimize this further with direct access into
    // the pixel buffers?
    fn draw_line_vertical(
        &mut self,
        x: XCord,
        y0: YCord,
        y1: YCord,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
    ) {
        let (start, end) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

        let width = self.width() as usize;
        let start_index = (start.0 * width) + x.0;
        let pixel_count = (end.0 - start.0) + 1;
        let color = self.get_color_as_pixel_data(color_index, palette_index);

        self.frame_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .skip(start_index)
            .step_by(width)
            .take(pixel_count)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    fn draw_line_horizontal(
        &mut self,
        x0: XCord,
        x1: XCord,
        y: YCord,
        color_index: ColorIndex,
        palette_index: PaletteIndex,
    ) {
        let (start, end) = if x0 < x1 { (x0, x1) } else { (x1, x0) };

        let start_index = (y.0 * self.width() as usize) + start.0;
        let pixel_count = (end.0 - start.0) + 1;
        let color = self.get_color_as_pixel_data(color_index, palette_index);

        self.frame_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .skip(start_index)
            .take(pixel_count)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }
}
