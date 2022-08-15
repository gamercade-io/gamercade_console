use std::sync::Arc;

use gamercade_audio::SoundRomInstance;

use crate::api::AudioApi;

pub struct AudioContext {
    sound_rom: Arc<SoundRomInstance>,
}

impl AudioContext {
    pub fn new(sound_rom: &Arc<SoundRomInstance>) -> Self {
        Self {
            sound_rom: sound_rom.clone(),
        }
    }
}

impl AudioApi for AudioContext {
    fn play_bgm(&self, _bgm_index: i32) {
        println!("TODO: play_bgm");
    }

    fn play_sfx(&self, sfx_index: i32, channel: i32) {
        println!("TODO: play_sfx");
    }

    fn stop_bgm(&self) {
        println!("TODO: stop_bgm");
    }

    fn stop_sfx(&self, channel: i32) {
        println!("TODO: stop_sfx");
    }

    fn bgm_is_playing(&self) -> i32 {
        println!("TODO: bgm_is_playing");
        -1
    }

    fn sfx_is_playing(&self, channel: i32) -> i32 {
        println!("TODO: sfx_is_playing");
        -1
    }
}
