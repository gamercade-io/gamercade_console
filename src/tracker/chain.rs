use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::CHAIN_MAX_PHRASE_COUNT;

use super::phrase::PhraseId;

/// Newtype Chain Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChainId(pub usize);

/// A chain is a series of phrases, which when combined together form a song.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    entries: ArrayVec<PhraseId, CHAIN_MAX_PHRASE_COUNT>,
}
