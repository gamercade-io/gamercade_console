use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{ColorIndex, PaletteIndex, SpriteIter};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpriteSheetIndex(pub u8);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpriteIndex(pub u8);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpriteSheet {
    pub height: usize,
    pub width: usize,
    pub sprites: Box<[ColorIndex]>,
    pub count: u8,
    pub default_palette: PaletteIndex,
}

impl Default for SpriteSheet {
    fn default() -> Self {
        let sprites = (0..16)
            .map(ColorIndex)
            .collect::<Vec<ColorIndex>>()
            .into_boxed_slice();

        Self {
            height: 4,
            width: 4,
            sprites,
            count: 1,
            default_palette: PaletteIndex(0),
        }
    }
}

impl SpriteSheet {
    pub fn count(&self) -> usize {
        self.sprites.len()
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        todo!();
    }

    pub fn step(&self) -> usize {
        self.width * self.height
    }

    pub fn iter_sprites(&self) -> SpriteIter {
        SpriteIter::new(self)
    }
}

impl Index<SpriteIndex> for SpriteSheet {
    type Output = [ColorIndex];

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        let step = self.step();
        let index = index.0 as usize;
        &self.sprites[step * index..step * (index + 1)]
    }
}
