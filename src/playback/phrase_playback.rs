use crate::{InstrumentInstance, PhraseId, SoundEngine, TrackerFlow, PHRASE_MAX_ENTRIES};

#[derive(Debug, Clone)]
pub struct PhrasePlayback {
    entry_index: usize,
    phrase: PhraseId,
}

impl PhrasePlayback {
    pub fn new(phrase: PhraseId) -> Self {
        Self {
            entry_index: 0,
            phrase,
        }
    }

    pub fn adjust_instrument_instance(
        &self,
        engine: &SoundEngine,
        instance: &mut InstrumentInstance,
    ) {
        if let Some(Some(next)) = &engine[self.phrase].entries.get(self.entry_index) {
            instance.update_from_phrase_entry(next, engine)
        }
    }

    pub fn next_step(&mut self) -> TrackerFlow {
        self.entry_index += 1;
        if self.entry_index >= PHRASE_MAX_ENTRIES {
            self.entry_index = 0;
            TrackerFlow::Finished
        } else {
            TrackerFlow::Continue
        }
    }
}
