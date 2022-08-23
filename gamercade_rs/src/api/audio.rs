use crate::raw;
use gamercade_audio::{SFX_CHANNELS, TOTAL_NOTES_COUNT};

/// Starts playing the passed in BGM index.
/// If a song is already playing, it will stop.
/// If the index is invalid, the song will also stop.
pub fn play_bgm(bgm_index: usize) {
    unsafe { raw::play_bgm(bgm_index as i32) }
}

/// Plays a sound effect on the specified channel.
/// If the sound effect index isn't valid, instead the channel will be muted.
/// An invalid channel will have no effect.
pub fn play_sfx(sfx_index: usize, channel: usize) {
    if channel < SFX_CHANNELS {
        unsafe { raw::play_sfx(sfx_index as i32, channel as i32) }
    }
}

/// Returns true if a BGM is playing.
pub fn bgm_is_active() -> bool {
    unsafe { raw::bgm_is_active() == 1 }
}

/// If the channel is valid, returns Some true or false value if
/// the channel is playing a sound.
/// If the channel index is invalid, returns None.
pub fn channel_is_active(channel: usize) -> Option<bool> {
    if channel < SFX_CHANNELS {
        unsafe {
            match raw::channel_is_active(channel as i32) {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    } else {
        None
    }
}

/// Stops the BGM from playing.
pub fn stop_bgm() {
    unsafe { raw::stop_bgm() }
}

/// Stops the channel from playing. If the channel index is invalid,
/// it will have no effect.
pub fn stop_channel(channel: usize) {
    if channel < SFX_CHANNELS {
        unsafe { raw::stop_channel(channel as i32) }
    }
}

/// Plays a note (a pre-determined frequency) using the specified instrument on the
/// specified channel. If the note, instrument index, or channel are invalid, does nothing.
/// Notes range from 0 to 95, starting from C1 until B9. If you want to play a specific frequency,
/// instead see play_frequency.
pub fn play_note(note_id: usize, instrument_index: usize, channel: usize) {
    if channel < SFX_CHANNELS && note_id < TOTAL_NOTES_COUNT {
        unsafe { raw::play_note(note_id as i32, instrument_index as i32, channel as i32) }
    }
}

/// Plays a note at a passed in frequency using the specified instrument on the
/// specified channel. If the instrument inde or channel are invalid, does nothing.
/// If you want to play a specific note by index, see play_note.
pub fn play_frequency(frequency: f32, instrument_index: usize, channel: usize) {
    if channel < SFX_CHANNELS {
        unsafe { raw::play_frequency(frequency, instrument_index as i32, channel as i32) }
    }
}
