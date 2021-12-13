use crate::core::Rom;

pub struct DrawContext<'a> {
    pub frame: &'a mut [u8],
    pub rom: &'a Rom,
}

impl DrawContext<'_> {
    fn clear_screen(&mut self, color_index: usize, palette_index: usize) {
        let color = self.get_color_as_pixel_data(color_index, palette_index);

        self.frame
            .chunks_exact_mut(4)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    fn set_pixel(&mut self, x: u32, y: u32, color_index: usize, palette_index: usize) {
        let pixel_index = self.x_y_to_pixel(x, y);
        let color = self.get_color_as_pixel_data(color_index, palette_index);

        self.frame[pixel_index..pixel_index + 4].copy_from_slice(&color);
    }

    fn x_y_to_pixel(&self, x: u32, y: u32) -> usize {
        ((x + (y * self.rom.resolution.width())) * 4) as usize
    }

    fn get_color_as_pixel_data(&self, color_index: usize, palette_index: usize) -> [u8; 4] {
        let color = self.rom.palettes[palette_index].colors[color_index];
        [color.r, color.g, color.b, 0xff]
    }
}
