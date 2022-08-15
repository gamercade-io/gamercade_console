use std::sync::Arc;

use crossbeam_channel::Sender;

use crate::{
    InstrumentChannelType, PhraseId, SoundEngine, TrackerFlow, TrackerState, PHRASE_MAX_ENTRIES,
};

#[derive(Debug)]
pub struct PhrasePlayback {
    pub(crate) engine: Arc<SoundEngine>,
    pub(crate) step_index: usize,
    pub(crate) phrase: Option<PhraseId>,
    pub(crate) sender: Sender<InstrumentChannelType>,
}

impl PhrasePlayback {
    pub(crate) fn new(
        phrase: Option<PhraseId>,
        sender: Sender<InstrumentChannelType>,
        engine: &Arc<SoundEngine>,
    ) -> Self {
        let mut out = Self {
            step_index: 0,
            phrase,
            engine: engine.clone(),
            sender,
        };

        out.notify_sources();
        out
    }

    /// Sets the active phrase ID for this playback
    /// and notifies the sound thread.
    pub(crate) fn set_phrase_id(&mut self, phrase: Option<PhraseId>) {
        self.phrase = phrase;
        self.step_index = 0;

        self.notify_sources();
    }

    /// Notifies sound thread of any updates to
    /// instrument, frequency, effects, etc
    fn notify_sources(&mut self) {
        if let Some(phrase_id) = self.phrase {
            if let Some(next_entry) = &self.engine[phrase_id].entries[self.step_index] {
                let out_message = InstrumentChannelType::new(next_entry, &self.engine);
                self.sender.try_send(out_message).unwrap();
            }
        }
    }

    /// Updates this phrase to match that of the passed in TrackerState
    /// Useful when trying to seek to an exact time.
    pub(crate) fn set_from_tracker_state(&mut self, tracker_state: &TrackerState) {
        self.phrase = tracker_state.phrase_id;
        self.step_index = tracker_state.phrase_step_index;

        self.notify_sources();
    }

    /// Increments the index and notifies the sound thread
    pub(crate) fn update_tracker(&mut self) -> TrackerFlow {
        self.step_index += 1;

        if self.step_index >= PHRASE_MAX_ENTRIES {
            self.step_index = 0;
            TrackerFlow::Finished
        } else {
            self.notify_sources();
            TrackerFlow::Continue
        }
    }
}
