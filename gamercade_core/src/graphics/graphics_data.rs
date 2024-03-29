use serde::{Deserialize, Serialize};

use crate::{Color, ColorIndex, SpriteIndex};

use super::{Palette, PaletteIndex, SpriteSheet, SpriteSheetIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsData {
    pub sprite_sheets: Box<[SpriteSheet]>,
    pub palettes: Box<[Palette]>,
}

impl GraphicsData {
    pub fn validate_palette_index(&self, index: i32) -> Result<PaletteIndex, &'static str> {
        if index >= 0 && index < self.palettes.len() as i32 {
            Ok(PaletteIndex(index as u8))
        } else {
            Err("invalid palette index")
        }
    }

    pub fn validate_sprite_sheet_index(
        &self,
        index: i32,
    ) -> Result<SpriteSheetIndex, &'static str> {
        if index >= 0 && index < self.sprite_sheets.len() as i32 {
            Ok(SpriteSheetIndex(index as u8))
        } else {
            Err("invalid sprite sheet index index")
        }
    }

    pub fn validate_sheet_and_sprite(
        &self,
        sheet_index: i32,
        sprite_index: i32,
    ) -> Result<(SpriteSheetIndex, SpriteIndex), &'static str> {
        let sheet = self.validate_sprite_sheet_index(sheet_index)?;
        self.validate_sprite_index(sheet, sprite_index)
            .map(|sprite| (sheet, sprite))
    }

    pub fn validate_sprite_index(
        &self,
        sheet_index: SpriteSheetIndex,
        index: i32,
    ) -> Result<SpriteIndex, &'static str> {
        if index >= 0 && index < self.sprite_sheets[sheet_index.0 as usize].count as i32 {
            Ok(SpriteIndex(index as u8))
        } else {
            Err("invalid sprite index index")
        }
    }

    pub fn palette(&self, palette_index: PaletteIndex) -> Option<&Palette> {
        self.palettes.get(palette_index.0 as usize)
    }

    pub fn sprite_sheet(&self, sprite_sheet_index: SpriteSheetIndex) -> Option<&SpriteSheet> {
        self.sprite_sheets.get(sprite_sheet_index.0 as usize)
    }

    pub fn color(&self, palette_index: PaletteIndex, color: ColorIndex) -> Option<&Color> {
        if let Some(palette) = self.palettes.get(palette_index.0 as usize) {
            palette.colors.get(color.0 as usize)
        } else {
            None
        }
    }
}

impl Default for GraphicsData {
    fn default() -> Self {
        Self {
            sprite_sheets: vec![SpriteSheet::default()].into_boxed_slice(),
            palettes: Palette::default_palette_collection()
                .into_iter()
                .map(|x| x.0)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}
