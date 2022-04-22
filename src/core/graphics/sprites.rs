use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::core::{ColorIndex, PaletteIndex};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteSheetIndex(pub(crate) usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub(crate) usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub height: usize,
    pub width: usize,
    sprites: Box<[Sprite]>,
}

impl SpriteSheet {
    pub fn count(&self) -> usize {
        self.sprites.len()
    }
}

impl Index<SpriteSheetIndex> for SpriteSheet {
    type Output = Sprite;

    fn index(&self, index: SpriteSheetIndex) -> &Self::Output {
        &self.sprites[index.0]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub data: Box<[ColorIndex]>,
    pub default_palette: PaletteIndex,
}
