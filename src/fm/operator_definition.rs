use crate::Waveform;

use super::FrequencyMultiplier;

pub struct OperatorDefinition {
    pub waveform: Waveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
}
