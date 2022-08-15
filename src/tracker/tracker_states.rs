use crate::{
    ChainId, ChainPlayback, PhraseId, SfxPlayback, SongId, SongPlayback, SFX_CHANNELS,
    SONG_TRACK_CHANNELS,
};

#[derive(Debug, Clone)]
pub struct TickerState {
    pub(crate) remaining: i64,
    pub(crate) reset: i64,
}

pub struct TrackerState {
    pub(crate) bgm: BgmState,
    pub(crate) sfx: [ChainState; SFX_CHANNELS],
}

#[derive(Debug, Clone)]
pub struct BgmState {
    pub(crate) song_id: Option<SongId>,
    pub(crate) chain_index: usize,
    pub(crate) chain_states: [ChainState; SONG_TRACK_CHANNELS],
    pub(crate) bgm_ticker: TickerState,
}

impl BgmState {
    pub fn new(bgm: &SongPlayback) -> Self {
        use std::array::from_fn;

        Self {
            song_id: bgm.song,
            chain_index: bgm.chain_index,
            chain_states: from_fn(|i| ChainState::new(&bgm.tracks[i])),
            bgm_ticker: bgm.ticker.as_state(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChainState {
    pub(crate) chain_id: Option<ChainId>,
    pub(crate) chain_phrase_index: usize,
    pub(crate) phrase_id: Option<PhraseId>,
    pub(crate) phrase_step_index: usize,
}

impl ChainState {
    pub fn new(chain: &ChainPlayback) -> Self {
        Self {
            chain_id: chain.chain,
            chain_phrase_index: chain.phrase_index,
            phrase_id: chain.phrase_playback.phrase,
            phrase_step_index: chain.phrase_playback.step_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SfxState {
    pub(crate) chain_state: ChainState,
    pub(crate) ticker: TickerState,
}

impl SfxState {
    pub fn new(sfx: &SfxPlayback) -> Self {
        Self {
            chain_state: ChainState::new(&sfx.chain_playback),
            ticker: sfx.ticker.as_state(),
        }
    }
}
