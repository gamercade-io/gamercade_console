use std::sync::Arc;

use gamercade_audio::{SongId, SONG_TRACK_CHANNELS};
use gamercade_sound_engine::{SoundEngineData, SoundRomInstance};

use crate::api::AudioApi;

pub struct AudioContext {
    sound_rom: Arc<SoundRomInstance>,
    pub sound_engine_data: SoundEngineData,
}

impl AudioContext {
    pub fn new(sound_rom: &Arc<SoundRomInstance>, output_sample_rate: usize) -> Self {
        Self {
            sound_rom: sound_rom.clone(),
            sound_engine_data: SoundEngineData::new(output_sample_rate, sound_rom),
        }
    }
}

impl AudioApi for AudioContext {
    fn play_bgm(&mut self, bgm_index: i32) {
        if let Ok(bgm_index) = usize::try_from(bgm_index) {
            if bgm_index < self.sound_rom.songs.len() {
                self.sound_engine_data.play_bgm(Some(SongId(bgm_index)));
            }
        }
    }

    fn play_sfx(&mut self, sfx_index: i32, channel: i32) {
        if let (Ok(sfx_index), Ok(channel)) = (usize::try_from(sfx_index), usize::try_from(channel))
        {
            if channel < SONG_TRACK_CHANNELS {
                self.sound_engine_data
                    .play_sfx(self.sound_rom.sfx.get(sfx_index).cloned(), channel);
            }
        }
    }

    fn bgm_is_active(&self) -> i32 {
        if self.sound_engine_data.bgm.song.is_some() {
            1
        } else {
            0
        }
    }

    fn channel_is_active(&self, channel: i32) -> i32 {
        if let Ok(channel) = usize::try_from(channel) {
            if channel < SONG_TRACK_CHANNELS {
                println!("TODO: channel_is_active is not implemented");
            }
        }

        -1 // Invalid Channel
    }

    fn stop_bgm(&mut self) {
        println!("TODO: stop_bgm is not implemented");
    }

    fn stop_channel(&mut self, channel: i32) {
        if let Ok(channel) = usize::try_from(channel) {
            if channel < SONG_TRACK_CHANNELS {
                println!("TODO: stop_channel is not implemented");
            }
        }
    }

    fn play_note(&mut self, _note_id: i32, _instrument_index: i32, _channel: i32) {
        println!("TODO: play_note is not implemented");
    }

    fn play_frequency(&mut self, _frequency: f32, _instrument_index: i32, _channel: i32) {
        println!("TODO: play_frequency is not implemented");
    }
}
