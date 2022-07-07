use std::mem::transmute;

const PALETTE_POSITION: u32 = 0;
const SPRITE_SHEET_POSITION: u32 = 8;
const SPRITE_INDEX_POSITION: u32 = 16;
const COLOR_INDEX_POSITION: u32 = 24;
const FLIP_X_POSITION: u32 = 30;
const FLIP_Y_POSITION: u32 = 31;

const MASK: u32 = u8::MAX as u32; // 8 bits
const COLOR_MASK: u32 = MASK >> 2; // 6 bits

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GraphicsParameters {
    pub palette_index: u8,
    pub sprite_sheet_index: u8,
    pub sprite_index: u8,
    pub color_index: u8,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl GraphicsParameters {
    pub fn palette_index(mut self, palette_index: u8) -> Self {
        self.palette_index = palette_index;
        self
    }

    pub fn sprite_sheet_index(mut self, sprite_sheet_index: u8) -> Self {
        self.sprite_sheet_index = sprite_sheet_index;
        self
    }

    pub fn sprite_index(mut self, sprite_index: u8) -> Self {
        self.sprite_index = sprite_index;
        self
    }

    pub fn color_index(mut self, color_index: u8) -> Self {
        assert!(color_index < 64);
        self.color_index = color_index;
        self
    }

    pub fn flip_x(mut self, flip_x: bool) -> Self {
        self.flip_x = flip_x;
        self
    }

    pub fn flip_y(mut self, flip_y: bool) -> Self {
        self.flip_y = flip_y;
        self
    }
}

impl From<GraphicsParameters> for u32 {
    fn from(gp: GraphicsParameters) -> Self {
        let mut out = 0;

        out |= (gp.palette_index as u32) << PALETTE_POSITION;
        out |= (gp.sprite_sheet_index as u32) << SPRITE_SHEET_POSITION;
        out |= (gp.sprite_index as u32) << SPRITE_INDEX_POSITION;
        out |= (gp.color_index as u32) << COLOR_INDEX_POSITION;
        out |= (gp.flip_x as u32) << FLIP_X_POSITION;
        out |= (gp.flip_y as u32) << FLIP_Y_POSITION;

        out
    }
}

impl From<GraphicsParameters> for i32 {
    fn from(gp: GraphicsParameters) -> Self {
        unsafe { transmute::<u32, i32>(u32::from(gp)) }
    }
}

impl From<u32> for GraphicsParameters {
    fn from(bits: u32) -> Self {
        Self {
            palette_index: ((bits & MASK << PALETTE_POSITION) >> PALETTE_POSITION) as u8,
            sprite_sheet_index: ((bits & MASK << SPRITE_SHEET_POSITION) >> SPRITE_SHEET_POSITION)
                as u8,
            sprite_index: ((bits & MASK << SPRITE_INDEX_POSITION) >> SPRITE_INDEX_POSITION) as u8,
            color_index: ((bits & COLOR_MASK << COLOR_INDEX_POSITION) >> COLOR_INDEX_POSITION)
                as u8,
            flip_x: bits & 1 << FLIP_X_POSITION != 0,
            flip_y: bits & 1 << FLIP_Y_POSITION != 0,
        }
    }
}

impl From<i32> for GraphicsParameters {
    fn from(bits: i32) -> Self {
        GraphicsParameters::from(unsafe { transmute::<i32, u32>(bits) })
    }
}

#[test]
fn test_graphics_parameters() {
    let params = [
        GraphicsParameters {
            palette_index: 127,
            sprite_sheet_index: 255,
            sprite_index: 92,
            color_index: 43,
            flip_x: true,
            flip_y: false,
        },
        GraphicsParameters {
            palette_index: 43,
            sprite_sheet_index: 0,
            sprite_index: 127,
            color_index: 62,
            flip_x: false,
            flip_y: true,
        },
        GraphicsParameters {
            palette_index: 255,
            sprite_sheet_index: 255,
            sprite_index: 255,
            color_index: 63,
            flip_x: true,
            flip_y: true,
        },
        GraphicsParameters {
            palette_index: 1,
            sprite_sheet_index: 1,
            sprite_index: 1,
            color_index: 1,
            flip_x: false,
            flip_y: false,
        },
        GraphicsParameters {
            palette_index: 0,
            sprite_sheet_index: 0,
            sprite_index: 0,
            color_index: 0,
            flip_x: false,
            flip_y: false,
        },
    ];

    params.into_iter().for_each(|p| {
        let as_u32: u32 = p.into();
        let from_u32: GraphicsParameters = as_u32.into();

        let as_i32: i32 = p.into();
        let from_i32: GraphicsParameters = as_i32.into();

        assert_eq!(from_u32, p);
        assert_eq!(from_i32, p);
    });
}
