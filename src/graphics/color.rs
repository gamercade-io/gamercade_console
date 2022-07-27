use serde::{Deserialize, Serialize};

use crate::{BYTES_PER_PIXEL, PALETTE_COLORS};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorIndex(pub u8);

impl TryFrom<i32> for ColorIndex {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value < PALETTE_COLORS as i32 {
            Ok(Self(value as u8))
        } else {
            Err("invalid color index")
        }
    }

    type Error = &'static str;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: usize) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    pub fn into_pixel_data(&self) -> [u8; BYTES_PER_PIXEL] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}
