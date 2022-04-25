use serde::{Deserialize, Serialize};

use super::{Palette, PaletteIndex, SpriteSheet, SpriteSheetIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsData {
    sprite_sheets: Box<[SpriteSheet]>,
    palettes: Box<[Palette]>,
}

impl GraphicsData {
    pub fn validate_palette_index(&self, index: i32) -> Result<PaletteIndex, &'static str> {
        if index >= 0 && index < self.palette_count() as i32 {
            Ok(PaletteIndex(index as u8))
        } else {
            Err("invalid palette index")
        }
    }

    pub fn palette_count(&self) -> usize {
        self.palettes.len()
    }

    pub fn palette(&self, palette_index: PaletteIndex) -> &Palette {
        &self.palettes[palette_index.0 as usize]
    }

    pub fn sprite_sheet(&self, sprite_sheet_index: SpriteSheetIndex) -> &SpriteSheet {
        &self.sprite_sheets[sprite_sheet_index.0 as usize]
    }
}

impl Default for GraphicsData {
    fn default() -> Self {
        Self {
            sprite_sheets: vec![].into_boxed_slice(),
            palettes: Palette::default_palette_collection().into_boxed_slice(),
        }
    }
}
