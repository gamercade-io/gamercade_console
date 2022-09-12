use std::sync::Arc;

use gamercade_audio::{Sfx, Song};
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

    fn bgm_length_secs(&self, bgm_index: i32) -> f32 {
        self.get_bgm(bgm_index)
            .map(Song::get_length)
            .unwrap_or(f32::NAN)
    }

    fn bgm_length_frames(&self, bgm_index: i32) -> i32 {
        self.get_bgm(bgm_index)
            .map(|song| self.secs_to_frames(song.get_length()))
            .unwrap_or(-1)
    }

    fn sfx_length_secs(&self, sfx_index: i32) -> f32 {
        self.get_sfx(sfx_index)
            .map(Sfx::get_length)
            .unwrap_or(f32::NAN)
    }

    fn sfx_length_frames(&self, sfx_index: i32) -> i32 {
        self.get_sfx(sfx_index)
            .map(|sfx| self.secs_to_frames(sfx.get_length()))
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
            .flatten()
    }

    fn get_bgm(&self, bgm_index: i32) -> Option<&Song> {
        self.rom.sounds.songs.get(bgm_index as usize)
    }

    fn get_sfx(&self, sfx_index: i32) -> Option<&Sfx> {
        self.rom.sounds.sfx.get(sfx_index as usize)
    }

    fn secs_to_frames(&self, secs: f32) -> i32 {
        (secs / self.rom.frame_rate.frame_time()).ceil() as i32
    }
}
