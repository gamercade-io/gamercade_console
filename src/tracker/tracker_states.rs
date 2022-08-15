use crate::{ChainId, ChainPlayback, PhraseId, SongId, SongPlayback, SONG_TRACK_CHANNELS};

#[derive(Debug, Clone)]
pub struct BgmState {
    pub song_id: Option<SongId>,
    pub chain_index: usize,
    pub trackers: [TrackerState; SONG_TRACK_CHANNELS],
}

impl BgmState {
    pub fn new(bgm: &SongPlayback) -> Self {
        use std::array::from_fn;

        Self {
            song_id: bgm.song,
            chain_index: bgm.chain_index,
            trackers: from_fn(|i| TrackerState::new(&bgm.tracks[i])),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackerState {
    pub chain_id: Option<ChainId>,
    pub chain_phrase_index: usize,
    pub phrase_id: Option<PhraseId>,
    pub phrase_step_index: usize,
}

impl TrackerState {
    pub fn new(tracker: &ChainPlayback) -> Self {
        Self {
            chain_id: tracker.chain,
            chain_phrase_index: tracker.phrase_index,
            phrase_id: tracker.phrase_playback.phrase,
            phrase_step_index: tracker.phrase_playback.step_index,
        }
    }
}
