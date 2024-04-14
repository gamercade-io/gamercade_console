use std::sync::Arc;

use gamercade_core::SpriteSheet;
use gamercade_fs::Rom;

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

    fn dimensions(&self) -> (i32, i32) {
        (self.rom.width(), self.rom.height())
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

    fn sprite_dimensions(&self, sheet_index: i32) -> (i32, i32) {
        self.get_sprite_sheet(sheet_index)
            .map(|sheet| (sheet.width as i32, sheet.height as i32))
            .unwrap_or((-1, -1))
    }

    fn sprite_count(&self, sheet_index: i32) -> i32 {
        self.get_sprite_sheet(sheet_index)
            .map(|sheet| sheet.count as i32)
            .unwrap_or(-1)
    }

    fn bgm_length_secs(&self, bgm_index: i32) -> f32 {
        self.get_bgm_length_secs(bgm_index).unwrap_or(f32::NAN)
    }

    fn bgm_length_frames(&self, bgm_index: i32) -> i32 {
        self.get_bgm_length_secs(bgm_index)
            .map(|secs| self.secs_to_frames(secs))
            .unwrap_or(-1)
    }

    fn sfx_length_secs(&self, sfx_index: i32) -> f32 {
        self.get_sfx_length_secs(sfx_index).unwrap_or(f32::NAN)
    }

    fn sfx_length_frames(&self, sfx_index: i32) -> i32 {
        self.get_sfx_length_secs(sfx_index)
            .map(|secs| self.secs_to_frames(secs))
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

    fn get_bgm_length_secs(&self, bgm_index: i32) -> Option<f32> {
        let song = self.rom.sounds.songs.get(bgm_index as usize)?;
        Some(song.song_length_seconds(&self.rom.sounds.chains))
    }

    fn get_sfx_length_secs(&self, sfx_index: i32) -> Option<f32> {
        let sfx = self.rom.sounds.sfx.get(sfx_index as usize)?;
        let chain = self.rom.sounds.chains.get(sfx.chain.0)?.as_ref()?;
        Some(chain.chain_length_seconds(sfx.bpm))
    }

    fn secs_to_frames(&self, secs: f32) -> i32 {
        (secs / self.rom.frame_rate.frame_time()).ceil() as i32
    }
}
