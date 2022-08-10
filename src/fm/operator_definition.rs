use serde::{Deserialize, Serialize};

use super::FrequencyMultiplier;
use crate::{EnvelopeDefinition, FMWaveform, OPERATOR_COUNT};

#[derive(Clone, Serialize, Deserialize)]
pub struct OperatorDefinitionBundle {
    pub operators: [OperatorDefinition; OPERATOR_COUNT],
}

impl Default for OperatorDefinitionBundle {
    fn default() -> Self {
        let first_envelope = EnvelopeDefinition {
            total_level: u8::MAX,
            ..Default::default()
        };

        let first = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: 0,
            envlope_definition: first_envelope,
        };

        let others = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: 0,
            envlope_definition: EnvelopeDefinition::default(),
        };

        Self {
            operators: [first, others.clone(), others.clone(), others],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: i8,
    pub envlope_definition: EnvelopeDefinition,
}
