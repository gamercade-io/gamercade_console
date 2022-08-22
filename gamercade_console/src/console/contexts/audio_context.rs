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
    fn play_bgm(&self, bgm_index: i32) {
        println!("TODO: play_bgm is not implemented");
    }

    fn play_sfx(&self, sfx_index: i32, channel: i32) {
        println!("TODO: play_sfx is not implemented");
    }

    fn bgm_is_active(&self) -> i32 {
        println!("TODO: bgm_is_active is not implemented");
        -1
    }

    fn channel_is_active(&self, channel: i32) -> i32 {
        println!("TODO: channel_is_active is not implemented");
        -1
    }

    fn stop_bgm(&self) {
        println!("TODO: stop_bgm is not implemented");
    }

    fn stop_channel(&self, channel: i32) {
        println!("TODO: stop_channel is not implemented");
    }

    fn play_note(&self, note_id: i32, instrument_index: i32, channel: i32) {
        println!("TODO: play_note is not implemented");
    }

    fn play_frequency(&self, frequency: f32, instrument_index: i32, channel: i32) {
        println!("TODO: play_frequency is not implemented");
    }
}
