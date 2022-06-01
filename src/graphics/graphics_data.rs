use serde::{Deserialize, Serialize};

use crate::{Color, ColorIndex, SpriteIndex};

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

    pub fn palette_count(&self) -> usize {
        self.palettes.len()
    }

    pub fn palette(&self, palette_index: PaletteIndex) -> &Palette {
        &self.palettes[palette_index.0 as usize]
    }

    pub fn sprite_sheet(&self, sprite_sheet_index: SpriteSheetIndex) -> &SpriteSheet {
        &self.sprite_sheets[sprite_sheet_index.0 as usize]
    }

    pub fn color(&self, palette_index: PaletteIndex, color: ColorIndex) -> &Color {
        &self.palettes[palette_index.0 as usize][color]
    }
}

impl Default for GraphicsData {
    fn default() -> Self {
        Self {
            sprite_sheets: vec![SpriteSheet::default()].into_boxed_slice(),
            palettes: Palette::default_palette_collection().into_boxed_slice(),
        }
    }
}
