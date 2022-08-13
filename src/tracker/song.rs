use serde::{Deserialize, Serialize};

use crate::{SONG_TRACK_CHANNELS, ChainId};

pub struct SongId(pub usize);

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    bpm: f32,
    tracks: Box<[[Option<ChainId>; SONG_TRACK_CHANNELS]]>,
}
