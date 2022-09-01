use std::ops::Index;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{
    Chain, ChainId, EnvelopeDefinition, IndexInterpolator, InstrumentDataDefinition, InstrumentId,
    Phrase, PhraseId, Song, SongId, WavetableDefinition, WavetableGenerator, WavetableWaveform,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundRom {
    pub songs: Box<[Song]>,
    pub chains: Box<[Option<Chain>]>,
    pub phrases: Box<[Option<Phrase>]>,
    pub instruments: Box<[Option<InstrumentDataDefinition>]>,
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
        let default_sine_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            interpolator: IndexInterpolator::default(),
        });

        let default_phrase = Phrase::c_scale(InstrumentId(0));

        let default_chain = Chain {
            entries: ArrayVec::from_iter(Some(Some(PhraseId(0)))),
        };

        let default_sfx = Sfx {
            bpm: 120.0,
            chain: ChainId(0),
        };

        Self {
            songs: vec![].into_boxed_slice(),
            chains: vec![Some(default_chain)].into_boxed_slice(),
            phrases: vec![Some(default_phrase)].into_boxed_slice(),
            instruments: vec![Some(default_sine_wave)].into_boxed_slice(),
            sfx: vec![default_sfx].into_boxed_slice(),
        }
    }
}

impl Index<SongId> for SoundRom {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}
