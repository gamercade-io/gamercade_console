use crate::{InstrumentInstance, PhraseId, SoundEngine, TrackerFlow, PHRASE_MAX_ENTRIES};

#[derive(Debug, Clone)]
pub struct PhrasePlayback {
    index: usize,
    phrase: PhraseId,
}

impl PhrasePlayback {
    pub fn new(phrase: PhraseId) -> Self {
        Self { index: 0, phrase }
    }

    pub fn adjust_instrument_instance(
        &self,
        engine: &SoundEngine,
        instance: &mut InstrumentInstance,
    ) {
        match &engine[self.phrase].entries.get(self.index) {
            Some(Some(next)) => instance.update_from_phrase_entry(next),
            _ => (),
        }
    }

    pub fn next_step(&mut self) -> TrackerFlow {
        self.index += 1;
        if self.index >= PHRASE_MAX_ENTRIES {
            self.index = 0;
            TrackerFlow::Finished
        } else {
            TrackerFlow::Continue
        }
    }
}
