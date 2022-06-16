use std::sync::Arc;

use gamercade_core::{Rom, SpriteSheet};

use crate::api::DataApi;

#[derive(Clone)]
pub struct DataContext {
    rom: Arc<Rom>,
}

impl DataContext {
    pub fn new(rom: Arc<Rom>) -> Self {
        Self { rom }
    }
}

impl DataApi for DataContext {
    fn height(&self) -> i32 {
        self.rom.height()
    }

    fn width(&self) -> i32 {
        self.rom.width()
    }

    fn fps(&self) -> i32 {
        self.rom.frame_rate.frames_per_second() as i32
    }

    fn frame_time(&self) -> f32 {
        self.rom.frame_rate.frame_time()
    }

    fn sprite_sheet_count(&self) -> i32 {
        self.rom.graphics.sprite_sheets.len() as i32
    }

    fn palette_count(&self) -> i32 {
        self.rom.graphics.palettes.len() as i32
    }

    fn sprite_height(&self, sheet_index: i32) -> i32 {
        self.get_sprite_sheet(sheet_index)
            .map(|sheet| sheet.height as i32)
            .unwrap_or(-1)
    }

    fn sprite_width(&self, sheet_index: i32) -> i32 {
        self.get_sprite_sheet(sheet_index)
            .map(|sheet| sheet.width as i32)
            .unwrap_or(-1)
    }

    fn sprite_count(&self, sheet_index: i32) -> i32 {
        self.get_sprite_sheet(sheet_index)
            .map(|sheet| sheet.count as i32)
            .unwrap_or(-1)
    }
}

impl DataContext {
    fn get_sprite_sheet(&self, sheet_index: i32) -> Option<&SpriteSheet> {
        self.rom
            .graphics
            .validate_sprite_sheet_index(sheet_index)
            .map(|index| self.rom.graphics.sprite_sheet(index))
            .ok()
    }
}
