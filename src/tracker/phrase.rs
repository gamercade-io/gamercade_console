use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use super::{effect::Effect, instrument::InstrumentId};
use crate::{EFFECT_COUNT, PHRASE_MAX_ENTRIES, NoteId};

/// Newtype Chain Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhraseId(pub usize);

/// A phrase is a series of notes tied to instruments, which when combined together form a chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub entries: ArrayVec<Option<PhraseEntry>, PHRASE_MAX_ENTRIES>,
}

impl Default for Phrase {
    fn default() -> Self {
        Self { entries: ArrayVec::from(std::array::from_fn(|_| None))}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An entry in the phrase, contains all data necessary to produce a sound
pub struct PhraseEntry {
    note: NoteId,
    instrument: InstrumentId,
    effects: [Effect; EFFECT_COUNT],
}
