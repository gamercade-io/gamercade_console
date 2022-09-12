use serde::{Deserialize, Serialize};

use crate::{ChainId, DEFAULT_BPM, SONG_TRACK_CHANNELS};

#[derive(Debug, Clone, Copy)]
pub struct SongId(pub usize);

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub bpm: f32,
    pub tracks: Box<[[Option<ChainId>; SONG_TRACK_CHANNELS]]>,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            tracks: vec![std::array::from_fn(|_| None)].into_boxed_slice(),
        }
    }
}

impl Song {
    /// Returns the length of the song in seconds
    pub fn get_length(&self) -> f32 {
        todo!()
    }
}
