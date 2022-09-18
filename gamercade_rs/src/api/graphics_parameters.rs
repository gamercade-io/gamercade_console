/// Newtype that represents bitpacked graphics parameters.
#[derive(Default, Clone, Copy)]
pub struct GraphicsParameters(pub i32);

const PALETTE_POSITION: i32 = 0;
const SPRITE_SHEET_POSITION: i32 = 8;
const SPRITE_INDEX_POSITION: i32 = 16;
const COLOR_INDEX_POSITION: i32 = 24;
const FLIP_X_POSITION: i32 = 30;
const FLIP_Y_POSITION: i32 = 31;

const MASK: i32 = u8::MAX as i32; // 8 bits
const COLOR_MASK: i32 = MASK >> 2; // 6 bits

impl GraphicsParameters {
    /// Generates a new Graphics Parameters with the default value.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Sets the palette index to the desired value.
    pub const fn palette_index(mut self, palette_index: u8) -> Self {
        self.0 &= !(MASK << PALETTE_POSITION);
        self.0 |= (palette_index as i32) << PALETTE_POSITION;
        self
    }

    /// Sets the Sprite Sheet Index to the desired value.
    pub const fn sprite_sheet_index(mut self, sprite_sheet_index: u8) -> Self {
        self.0 &= !(MASK << SPRITE_SHEET_POSITION);
        self.0 |= (sprite_sheet_index as i32) << SPRITE_SHEET_POSITION;
        self
    }

    /// Sets the Sprite Index to the desired value.
    pub const fn sprite_index(mut self, sprite_index: u8) -> Self {
        self.0 &= !(MASK << SPRITE_INDEX_POSITION);
        self.0 |= (sprite_index as i32) << SPRITE_INDEX_POSITION;
        self
    }

    /// Sets the Color Index to the desired value.
    pub const fn color_index(mut self, color_index: u8) -> Self {
        let color_index = color_index as i32 & COLOR_MASK;
        self.0 &= !(COLOR_MASK << COLOR_INDEX_POSITION);
        self.0 |= (color_index as i32) << COLOR_INDEX_POSITION;
        self
    }

    /// Sets the Flip X to the desired value.
    pub const fn flip_x(mut self, flip_x: bool) -> Self {
        let flip_x = flip_x as i32;
        self.0 &= !(flip_x << FLIP_X_POSITION);
        self.0 |= flip_x << FLIP_X_POSITION;
        self
    }

    /// Sets the Flip Y to the desired value
    pub const fn flip_y(mut self, flip_y: bool) -> Self {
        let flip_y = flip_y as i32;
        self.0 &= !(flip_y << FLIP_Y_POSITION);
        self.0 |= flip_y << FLIP_Y_POSITION;
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_graphics_parameters() {
        use super::GraphicsParameters;

        let parameter1 = GraphicsParameters::default()
            .color_index(255)
            .palette_index(255);
        let parameter2 = parameter1.color_index(5).color_index(255);

        assert_eq!(parameter1.0, parameter2.0);

        let parameter1 = parameter1.color_index(2);
        let parameter2 = parameter2.color_index(2);
        assert_eq!(parameter1.0, parameter2.0);
    }
}
