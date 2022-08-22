use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{
    Chain, ChainId, InstrumentDataDefinition, InstrumentId, Phrase, PhraseId, Song, SongId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundRom {
    pub songs: Box<[Song]>,
    pub chains: Box<[Chain]>,
    pub phrases: Box<[Phrase]>,
    pub instruments: Box<[InstrumentDataDefinition]>,
    pub sfx: Box<[Sfx]>,
}

/// Represents a singular sound effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sfx {
    pub bpm: f32,
    pub chain: ChainId,
    // TODO: Should we include other data here, like
    // loop style? or should this be handled by game code?
}

impl Default for SoundRom {
    fn default() -> Self {
        Self {
            songs: vec![].into_boxed_slice(),
            chains: vec![].into_boxed_slice(),
            phrases: vec![].into_boxed_slice(),
            instruments: vec![].into_boxed_slice(),
            sfx: vec![].into_boxed_slice(),
        }
    }
}

impl Index<SongId> for SoundRom {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}

impl Index<ChainId> for SoundRom {
    type Output = Chain;

    fn index(&self, index: ChainId) -> &Self::Output {
        &self.chains[index.0]
    }
}

impl Index<PhraseId> for SoundRom {
    type Output = Phrase;

    fn index(&self, index: PhraseId) -> &Self::Output {
        &self.phrases[index.0]
    }
}

impl Index<InstrumentId> for SoundRom {
    type Output = InstrumentDataDefinition;

    fn index(&self, index: InstrumentId) -> &Self::Output {
        &self.instruments[index.0]
    }
}
