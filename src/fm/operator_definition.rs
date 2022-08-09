use crate::FMWaveform;

use super::FrequencyMultiplier;

pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
}
