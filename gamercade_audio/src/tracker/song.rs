use serde::{Deserialize, Serialize};

use crate::{ChainId, SONG_TRACK_CHANNELS};

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
            bpm: 120.0,
            tracks: vec![std::array::from_fn(|_| None)].into_boxed_slice(),
        }
    }
}
