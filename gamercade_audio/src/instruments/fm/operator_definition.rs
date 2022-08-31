use serde::{Deserialize, Serialize};

use super::FrequencyMultiplier;
use crate::{Detune, EnvelopeDefinition, EnvelopeValue, FMWaveform, OPERATOR_COUNT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDefinitionBundle {
    pub operators: [OperatorDefinition; OPERATOR_COUNT],
}

impl Default for OperatorDefinitionBundle {
    fn default() -> Self {
        let modulators_envelope = EnvelopeDefinition {
            total_level: EnvelopeValue::zero(),
            ..Default::default()
        };

        let silent_modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: modulators_envelope,
        };

        let modulator_envelope = EnvelopeDefinition {
            total_level: EnvelopeValue(170), // Random value to compare against audio-test
            ..EnvelopeDefinition::interesting()
        };

        let modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: modulator_envelope,
        };

        let carrier = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: EnvelopeDefinition::interesting(),
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
    pub detune: Detune,
    pub envlope_definition: EnvelopeDefinition,
}
