use std::sync::Arc;

use crate::{EnvelopeDefinition, FMWaveform};

use super::FrequencyMultiplier;

#[derive(Clone)]
pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
    pub envlope_definition: Arc<EnvelopeDefinition>,
}
