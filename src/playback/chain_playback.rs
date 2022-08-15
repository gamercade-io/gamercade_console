use std::sync::Arc;

use crossbeam_channel::Sender;

use crate::{
    ChainId, InstrumentChannelType, PhrasePlayback, SoundRomInstance, TrackerFlow, TrackerState,
};

#[derive(Debug)]
pub struct ChainPlayback {
    pub(crate) engine: Arc<SoundRomInstance>,
    pub(crate) phrase_index: usize,
    pub(crate) chain: Option<ChainId>,
    pub(crate) phrase_playback: PhrasePlayback,
}

impl ChainPlayback {
    pub fn new(
        chain: Option<ChainId>,
        sender: Sender<InstrumentChannelType>,
        engine: &Arc<SoundRomInstance>,
    ) -> Self {
        let mut out = Self {
            engine: engine.clone(),
            phrase_index: 0,
            chain,
            phrase_playback: PhrasePlayback::new(None, sender, engine),
        };

        out.set_chain_id(chain);
        out
    }

    /// Updates this chain to match that of the passed in TrackerState
    /// Useful when trying to seek to an exact time.
    pub fn set_from_tracker_state(&mut self, tracker_state: &TrackerState) {
        self.chain = tracker_state.chain_id;
        self.phrase_index = tracker_state.phrase_step_index;

        self.phrase_playback.set_from_tracker_state(tracker_state)
    }

    /// Sets the active chain ID for this playback
    /// and notifies the sound thread. This will additionally
    /// set the reset phrase index to zero.
    pub fn set_chain_id(&mut self, chain: Option<ChainId>) {
        self.chain = chain;
        self.phrase_index = 0;

        let phrase_id = chain.and_then(|chain| self.engine[chain].entries[0]);

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

            let next_phrase = self.engine[chain]
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
