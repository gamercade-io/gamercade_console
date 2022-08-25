use std::sync::Arc;

use gamercade_audio::{SongId, SFX_CHANNELS, SONG_TRACK_CHANNELS, TOTAL_NOTES_COUNT};
use gamercade_sound_engine::{SoundEngineData, SoundRomInstance};

use crate::api::AudioApi;

pub struct AudioContext {
    sound_rom: Arc<SoundRomInstance>,
    pub sound_engine_data: SoundEngineData,
    pub changed: bool,
}

impl AudioContext {
    pub fn new(sound_rom: &Arc<SoundRomInstance>, output_sample_rate: usize) -> Self {
        Self {
            sound_rom: sound_rom.clone(),
            sound_engine_data: SoundEngineData::new(output_sample_rate, sound_rom),
            changed: false,
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

    // TODO: Improve this to check if the state is actually playing a sound or not,
    // parhaps via TrackerFlow
    fn bgm_is_active(&self) -> i32 {
        if self.sound_engine_data.bgm.song.is_some() {
            1
        } else {
            0
        }
    }

    // TODO: Improve this to actually check if the channel is playing a sound or not,
    // perhals via TrackerFlow
    fn channel_is_active(&self, channel: i32) -> i32 {
        if let Ok(channel) = usize::try_from(channel) {
            if channel < SONG_TRACK_CHANNELS {
                println!("TODO: channel_is_active is not implemented");
            }
        }

        -1 // Invalid Channel
    }

    fn stop_bgm(&mut self) {
        self.sound_engine_data.play_bgm(None);
    }

    fn stop_channel(&mut self, channel: i32) {
        if let Ok(channel) = usize::try_from(channel) {
            if channel < SONG_TRACK_CHANNELS {
                self.sound_engine_data.play_sfx(None, channel)
            }
        }
    }

    fn play_note(&mut self, note_id: i32, instrument_index: i32, channel: i32) {
        let valid_note = note_id >= 0 && note_id < TOTAL_NOTES_COUNT as i32;

        let instrument_index = usize::try_from(instrument_index);
        let channel = usize::try_from(channel);

        if let (true, Ok(instrument_index), Ok(channel)) = (valid_note, instrument_index, channel) {
            if channel < SFX_CHANNELS {
                self.sound_engine_data
                    .play_note(note_id, instrument_index, channel);
            }
        };
    }

    fn play_frequency(&mut self, frequency: f32, instrument_index: i32, channel: i32) {
        if let (Ok(instrument_index), Ok(channel)) =
            (usize::try_from(instrument_index), usize::try_from(channel))
        {
            if channel < SFX_CHANNELS {
                self.sound_engine_data
                    .play_frequency(frequency, instrument_index, channel);
            }
        }
    }
}
