use serde::{Deserialize, Serialize};

use super::FrequencyMultiplier;
use crate::{EnvelopeDefinition, FMWaveform, OPERATOR_COUNT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDefinitionBundle {
    pub operators: [OperatorDefinition; OPERATOR_COUNT],
}

impl Default for OperatorDefinitionBundle {
    fn default() -> Self {
        let modulators_envelope = EnvelopeDefinition {
            total_level: 0,
            ..Default::default()
        };

        let silent_modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: 0,
            envlope_definition: modulators_envelope,
        };

        let modulator_envelope = EnvelopeDefinition {
            total_level: 49_152, // Random value to compare against audio-test
            ..Default::default()
        };

        let modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: 0,
            envlope_definition: modulator_envelope,
        };

        let carrier = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: 0,
            envlope_definition: EnvelopeDefinition::default(),
        };

        Self {
            operators: [
                silent_modulator.clone(),
                silent_modulator,
                modulator,
                carrier,
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
    pub envlope_definition: EnvelopeDefinition,
}
