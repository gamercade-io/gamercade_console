use serde::{Deserialize, Serialize};

use crate::{PatchDefinition, WavetableDefinition};

/// Newtype Instrument Identifier
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InstrumentId(pub usize);

/// The types of instruments the tracker can use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentDefinition {
    Wavetable(WavetableDefinition),
    FMSynth(PatchDefinition),
}
