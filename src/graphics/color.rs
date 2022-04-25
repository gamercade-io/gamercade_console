use serde::{Deserialize, Serialize};

use crate::PALETTE_COLORS;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorIndex(pub(crate) usize);

impl TryFrom<i32> for ColorIndex {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value < PALETTE_COLORS as i32 {
            Ok(Self(value as usize))
        } else {
            Err("invalid color index")
        }
    }

    type Error = &'static str;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}
