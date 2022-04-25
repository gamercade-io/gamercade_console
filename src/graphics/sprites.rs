use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{ColorIndex, PaletteIndex};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteSheetIndex(pub(crate) u8);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub(crate) u8);

//TODO: Could this be optimized with a single slice of data ?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub height: usize,
    pub width: usize,
    sprites: Box<[Sprite]>,
    default_palette: Option<PaletteIndex>,
}

impl SpriteSheet {
    pub fn count(&self) -> usize {
        self.sprites.len()
    }

    pub fn test_palette_sprite_sheet() -> Self {
        let data = (0..16)
            .map(ColorIndex)
            .collect::<Vec<ColorIndex>>()
            .into_boxed_slice();

        Self {
            height: 4,
            width: 4,
            sprites: vec![Sprite { data }].into_boxed_slice(),
            default_palette: None,
        }
    }
}

impl Index<SpriteIndex> for SpriteSheet {
    type Output = Sprite;

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        &self.sprites[index.0 as usize]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sprite {
    pub data: Box<[ColorIndex]>,
}
