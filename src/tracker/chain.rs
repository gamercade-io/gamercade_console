use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::CHAIN_MAX_PHRASE_COUNT;

use super::phrase::PhraseId;

/// Newtype Chain Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainId(pub usize);

/// A chain is a series of phrases, which when combined together form a song.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub entries: ArrayVec<Option<PhraseId>, CHAIN_MAX_PHRASE_COUNT>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            entries: ArrayVec::from(std::array::from_fn(|_| None)),
        }
    }
}
