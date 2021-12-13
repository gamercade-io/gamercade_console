use serde::{Deserialize, Serialize};

use super::{ColorIndex, PaletteIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    height: usize,
    width: usize,
    data: Box<[ColorIndex]>,
    default_palette: PaletteIndex,
}
