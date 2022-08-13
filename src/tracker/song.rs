use serde::{Deserialize, Serialize};

use super::chain::Chain;
use crate::SONG_TRACK_CHANNELS;

pub struct SongId(pub usize);

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    tracks: [Box<[Chain]>; SONG_TRACK_CHANNELS],
}
