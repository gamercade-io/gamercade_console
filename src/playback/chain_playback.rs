use crate::{
    ChainId, InstrumentInstance, PhrasePlayback, SoundEngine, TrackerFlow, CHAIN_MAX_PHRASE_COUNT,
};

#[derive(Debug, Clone)]
pub struct ChainPlayback {
    index: usize,
    chain: ChainId,
    phrase_playback: Option<PhrasePlayback>,
}

impl ChainPlayback {
    pub fn new(chain: ChainId, engine: &SoundEngine) -> Self {
        let mut out = Self {
            index: 0,
            chain,
            phrase_playback: None,
        };

        out.fetch_phrase_playback(engine);
        out
    }

    fn fetch_phrase_playback(&mut self, engine: &SoundEngine) {
        self.phrase_playback = match engine[self.chain].entries.get(self.index) {
            Some(Some(next)) => Some(PhrasePlayback::new(*next)),
            _ => None,
        }
    }

    pub fn update_tracker(
        &mut self,
        engine: &SoundEngine,
        instance: &mut InstrumentInstance,
    ) -> TrackerFlow {
        match &mut self.phrase_playback {
            Some(phrase) => {
                phrase.adjust_instrument_instance(engine, instance);

                match phrase.next_step() {
                    TrackerFlow::Continue => TrackerFlow::Continue,
                    TrackerFlow::Finished => self.next_step(engine),
                }
            }
            None => TrackerFlow::Finished,
        }
    }

    pub fn next_step(&mut self, engine: &SoundEngine) -> TrackerFlow {
        self.index += 1;

        let out = if self.index >= CHAIN_MAX_PHRASE_COUNT {
            self.index = 0;
            TrackerFlow::Finished
        } else {
            TrackerFlow::Continue
        };

        self.fetch_phrase_playback(engine);
        out
    }
}
