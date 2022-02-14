use serde::{Deserialize, Serialize};

use crate::core::{ColorIndex, PaletteIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    height: usize,
    width: usize,
    data: Box<[ColorIndex]>,
    default_palette: PaletteIndex,
}
