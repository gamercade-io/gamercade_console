use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{ColorIndex, PaletteIndex};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteSheetIndex(pub(crate) u8);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub(crate) u8);

//TODO: Could this be optimized with a single slice of data ?
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpriteSheet {
    pub height: usize,
    pub width: usize,
    pub sprites: Box<[Sprite]>,
    pub default_palette: PaletteIndex,
}

impl Default for SpriteSheet {
    fn default() -> Self {
        let data = (0..16)
            .map(ColorIndex)
            .collect::<Vec<ColorIndex>>()
            .into_boxed_slice();

        Self {
            height: 4,
            width: 4,
            sprites: vec![Sprite { data }].into_boxed_slice(),
            default_palette: PaletteIndex(0),
        }
    }
}

impl SpriteSheet {
    pub fn count(&self) -> usize {
        self.sprites.len()
    }
}

impl Index<SpriteIndex> for SpriteSheet {
    type Output = Sprite;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        &self.sprites[index.0 as usize]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sprite {
    pub data: Box<[ColorIndex]>,
}
