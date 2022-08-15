use std::{ops::Index, sync::Arc};

use crate::{
    Chain, ChainId, InstrumentDefinition, InstrumentId, InstrumentKind, PatchDefinition, Phrase,
    PhraseId, Song, SongId, SoundRom, WavetableDefinition,
};

/// An engine loaded in memory, ready to use.
#[derive(Debug)]
pub struct SoundRomInstance {
    songs: Box<[Song]>,
    chains: Box<[Chain]>,
    phrases: Box<[Phrase]>,
    instrument_bank: Box<[Instrument]>,
}

/// An instrument stored in memory, ready to generate the pieces
/// needed to produce sounds.
#[derive(Clone, Debug)]
pub enum Instrument {
    Wavetable(Arc<WavetableDefinition>),
    FMSynth(Arc<PatchDefinition>),
}

impl Instrument {
    /// Returns the kind of the instrument
    pub fn get_type(&self) -> InstrumentKind {
        match self {
            Instrument::Wavetable(_) => InstrumentKind::Wavetable,
            Instrument::FMSynth(_) => InstrumentKind::FMSynth,
        }
    }
}

impl SoundRomInstance {
    /// Generates a new sound engine. This struct is used throughout the audio system.
    /// Performs some light logic to prepare the generation of sound sources.
    pub(crate) fn new(rom: SoundRom) -> Self {
        Self {
            songs: rom.songs,
            chains: rom.chains,
            phrases: rom.phrases,
            instrument_bank: Vec::from(rom.instruments)
                .into_iter()
                .map(|instrument| match instrument {
                    InstrumentDefinition::Wavetable(wavetable_def) => {
                        Instrument::Wavetable(Arc::new(wavetable_def))
                    }
                    InstrumentDefinition::FMSynth(fm_def) => Instrument::FMSynth(Arc::new(fm_def)),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl Index<SongId> for SoundRomInstance {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}

impl Index<ChainId> for SoundRomInstance {
    type Output = Chain;

    fn index(&self, index: ChainId) -> &Self::Output {
        &self.chains[index.0]
    }
}

impl Index<PhraseId> for SoundRomInstance {
    type Output = Phrase;

    fn index(&self, index: PhraseId) -> &Self::Output {
        &self.phrases[index.0]
    }
}

impl Index<InstrumentId> for SoundRomInstance {
    type Output = Instrument;

    fn index(&self, index: InstrumentId) -> &Self::Output {
        &self.instrument_bank[index.0]
    }
}
