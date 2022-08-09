use serde::{Deserialize, Serialize};

use super::FrequencyMultiplier;
use crate::{EnvelopeDefinition, FMWaveform};

#[derive(Clone, Serialize, Deserialize)]
pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
    pub envlope_definition: EnvelopeDefinition,
}
