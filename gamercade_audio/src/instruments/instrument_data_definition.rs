use serde::{Deserialize, Serialize};

use crate::{InstrumentKind, PatchDefinition, SampleDefinition, WavetableDefinition};

/// Newtype Instrument Identifier
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct InstrumentId(pub usize);

/// The types of instruments the tracker can use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentDataDefinition {
    Wavetable(WavetableDefinition),
    FMSynth(PatchDefinition),
    Sampler(SampleDefinition),
}

impl InstrumentDataDefinition {
    pub fn get_kind(&self) -> InstrumentKind {
        match self {
            InstrumentDataDefinition::Wavetable(_) => InstrumentKind::Wavetable,
            InstrumentDataDefinition::FMSynth(_) => InstrumentKind::FMSynth,
            InstrumentDataDefinition::Sampler(_) => InstrumentKind::Sampler,
        }
    }
}
