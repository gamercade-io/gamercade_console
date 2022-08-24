/// How many channels are available for a song.
pub const SONG_TRACK_CHANNELS: usize = 8;

/// How many channels are available for sfx.
pub const SFX_CHANNELS: usize = 8;

/// How many effects are available.
pub const EFFECT_COUNT: usize = 3;

// Maximum allowed phrases in a chain
pub const CHAIN_MAX_PHRASE_COUNT: usize = 16;

// Maximum allowed entries (or steps) in a phrase
pub const PHRASE_MAX_ENTRIES: usize = 16;

// I'm not sure why this is correct but it is
pub const PHRASE_STEPS_PER_BEAT: usize = 4;

use strum::EnumCount;

use crate::{NoteName, Octave};
/// The total number of valid notes. 96 notes from C1 -> B9
pub const TOTAL_NOTES_COUNT: usize = (Octave::COUNT - 1) * NoteName::COUNT;
