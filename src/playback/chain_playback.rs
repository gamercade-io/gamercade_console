use std::sync::Arc;

use rtrb::Producer;

use crate::{
    ChainId, ChainState, InstrumentChannelType, PhrasePlayback, SfxState, SoundRomInstance, Ticker,
    TrackerFlow,
};

#[derive(Debug)]
pub struct SfxPlayback {
    pub(crate) chain_playback: ChainPlayback,
    pub(crate) ticker: Arc<Ticker>,
}

impl SfxPlayback {
    pub(crate) fn set_from_sfx_state(&mut self, state: &SfxState) {
        self.chain_playback.set_from_chain_state(&state.chain_state);
        self.ticker.write_from_state(&state.ticker);
    }
}

#[derive(Debug)]
pub struct ChainPlayback {
    pub(crate) rom: Arc<SoundRomInstance>,
    pub(crate) phrase_index: usize,
    pub(crate) chain: Option<ChainId>,
    pub(crate) phrase_playback: PhrasePlayback,
}

impl ChainPlayback {
    pub fn new(
        chain: Option<ChainId>,
        producer: Producer<InstrumentChannelType>,
        rom: &Arc<SoundRomInstance>,
    ) -> Self {
        let mut out = Self {
            rom: rom.clone(),
            phrase_index: 0,
            chain,
            phrase_playback: PhrasePlayback::new(None, producer, rom),
        };

        out.set_chain_id(chain);
        out
    }

    /// Updates this chain to match that of the passed in TrackerState
    /// Useful when trying to seek to an exact time.
    pub(crate) fn set_from_chain_state(&mut self, chain_state: &ChainState) {
        self.chain = chain_state.chain_id;
        self.phrase_index = chain_state.chain_phrase_index;

        self.phrase_playback.set_from_chain_state(chain_state)
    }

    /// Sets the active chain ID for this playback
    /// and notifies the sound thread. This will additionally
    /// set the reset phrase index to zero.
    pub fn set_chain_id(&mut self, chain: Option<ChainId>) {
        self.chain = chain;
        self.phrase_index = 0;

        let phrase_id = chain.and_then(|chain| self.rom[chain].entries[0]);

        self.phrase_playback.set_phrase_id(phrase_id);
    }

    /// Calls update_tracker on the phrase playback,
    /// if its done, will increment our current phrase index
    /// within the chain
    pub fn update_tracker(&mut self) -> TrackerFlow {
        match self.phrase_playback.update_tracker() {
            TrackerFlow::Continue => TrackerFlow::Continue,
            TrackerFlow::Finished => self.next_step(),
        }
    }

    /// Advances the chain to the next phrase within the chain.
    fn next_step(&mut self) -> TrackerFlow {
        if let Some(chain) = self.chain {
            self.phrase_index += 1;

            let next_phrase = self.rom[chain]
                .entries
                .get(self.phrase_index)
                .and_then(|x| *x);

            if next_phrase.is_some() {
                self.phrase_playback.set_phrase_id(next_phrase);
                TrackerFlow::Continue
            } else {
                TrackerFlow::Finished
            }
        } else {
            TrackerFlow::Finished
        }
    }
}
