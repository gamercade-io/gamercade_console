use serde::{Deserialize, Serialize};

use crate::{ColorIndex, PaletteIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    height: usize,
    width: usize,
    data: Box<[ColorIndex]>,
    default_palette: PaletteIndex,
}
