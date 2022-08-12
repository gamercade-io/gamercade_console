use serde::{Deserialize, Serialize};

use super::{effect::Effect, instrument::InstrumentId};
use crate::EFFECT_COUNT;

/// Newtype Chain Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhraseId(pub usize);

/// A phrase is a series of notes tied to instruments, which when combined together form a chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub entries: Box<[PhraseEntry]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An entry in the phrase, contains all data necessary to produce a sound
pub struct PhraseEntry {
    note: usize,
    instrument: InstrumentId,
    effects: [Effect; EFFECT_COUNT],
}
