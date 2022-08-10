use serde::{Deserialize, Serialize};

use crate::EnvelopeType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeDefinition {
    /// The max level
    pub total_level: EnvelopeType,

    /// The level decayed to after the initial attack
    pub sustain_level: EnvelopeType,

    /// The length of time to finish the attack phase. Between zero and total_level
    pub attack_time: EnvelopeType,

    /// The length of time to finish the decay phase. Between total_level and sustain_level
    pub decay_attack_time: EnvelopeType,

    /// The length of time to finish sustain. Between sustain_level and zero.
    pub decay_sustain_time: EnvelopeType,

    /// The the length of decay after the key is released.
    pub release_time: EnvelopeType,
}

impl Default for EnvelopeDefinition {
    fn default() -> Self {
        Self {
            total_level: EnvelopeType::MAX,
            sustain_level: EnvelopeType::MAX,

            attack_time: 0,
            decay_attack_time: 0,
            decay_sustain_time: EnvelopeType::MAX,
            release_time: 0,
        }
    }
}

impl EnvelopeDefinition {
    pub fn interesting() -> Self {
        Self {
            total_level: EnvelopeType::MAX,
            sustain_level: EnvelopeType::MAX / 2,

            attack_time: 6,
            decay_attack_time: EnvelopeType::MAX / 512,
            decay_sustain_time: EnvelopeType::MAX / 8,
            release_time: 0,
        }
    }
}
