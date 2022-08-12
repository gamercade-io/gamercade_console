use serde::{Deserialize, Serialize};

use super::chain::Chain;
use crate::SONG_CHANNELS;

/// A song is a series of chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    chains: [Box<[Chain]>; SONG_CHANNELS],
}
