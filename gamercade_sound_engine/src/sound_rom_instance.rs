use std::{ops::Index, sync::Arc};

use gamercade_audio::{
    Chain, InstrumentDataDefinition, PatchDefinition, Phrase, SampleDefinition, Song, SoundRom,
};

use crate::{Sfx, SongId, WavetableDefinition};

/// An engine loaded in memory, ready to use.
#[derive(Debug)]
pub struct SoundRomInstance {
    pub songs: Box<[Song]>,
    pub chains: Box<[Chain]>,
    pub phrases: Box<[Phrase]>,
    pub instrument_bank: Box<[InstrumentDefinition]>,
    pub sfx: Box<[Sfx]>,
}

/// An instrument stored in memory, ready to generate the pieces
/// needed to produce sounds.
#[derive(Clone, Debug)]
pub struct InstrumentDefinition {
    pub id: usize,
    pub kind: InstrumentDefinitionKind,
}

#[derive(Clone, Debug)]
pub enum InstrumentDefinitionKind {
    Wavetable(Arc<WavetableDefinition>),
    FMSynth(Arc<PatchDefinition>),
    Sampler(Arc<SampleDefinition>),
}

impl From<InstrumentDataDefinition> for InstrumentDefinitionKind {
    fn from(data: InstrumentDataDefinition) -> Self {
        match data {
            InstrumentDataDefinition::Wavetable(wavetable_def) => {
                InstrumentDefinitionKind::Wavetable(Arc::new(wavetable_def))
            }
            InstrumentDataDefinition::FMSynth(fm_def) => {
                InstrumentDefinitionKind::FMSynth(Arc::new(fm_def))
            }
            InstrumentDataDefinition::Sampler(sample) => {
                InstrumentDefinitionKind::Sampler(Arc::new(sample))
            }
        }
    }
}

impl SoundRomInstance {
    /// Generates a new sound engine. This struct is used throughout the audio system.
    /// Performs some light logic to prepare the generation of sound sources.
    pub fn new(rom: &SoundRom) -> Self {
        Self {
            songs: rom.songs.clone(),
            chains: rom.chains.clone(),
            phrases: rom.phrases.clone(),
            instrument_bank: Vec::from(rom.instruments.clone())
                .into_iter()
                .enumerate()
                .map(|(index, instrument)| InstrumentDefinition {
                    id: index,
                    kind: InstrumentDefinitionKind::from(instrument),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            sfx: rom.sfx.clone(),
        }
    }
}

impl Index<SongId> for SoundRomInstance {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}
