use crate::{InstrumentInstance, PhraseId, SoundEngine, PHRASE_MAX_ENTRIES};

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
        match &engine[self.phrase].entries[self.index] {
            Some(next) => instance.update_from_phrase_entry(next),
            None => (),
        }
    }

    pub fn next_step(&mut self) {
        self.index += 1;
        if self.index >= PHRASE_MAX_ENTRIES {
            self.index = 0;
        }
    }
}
