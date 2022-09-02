use serde::{Deserialize, Serialize};

use crate::{ChainId, SONG_TRACK_CHANNELS};

#[derive(Debug, Clone, Copy)]
pub struct SongId(pub usize);

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Song {
    pub bpm: f32,
    pub tracks: Box<[[Option<ChainId>; SONG_TRACK_CHANNELS]]>,
}
