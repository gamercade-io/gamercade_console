use serde::{Deserialize, Serialize};

use super::{PALETTE_COLORS, PALETTE_COUNT};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PaletteIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorIndex(pub usize);

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
