use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeDefinition {
    /// The max level
    pub total_level: u8,

    /// The level decayed to after the initial attack
    pub sustain_level: u8,

    /// The rate of increase before reaching total_level. Between zero and total_level
    pub attack_rate: u8,

    /// The rate of decay after attacking. Between total_level and sustain_level
    pub decay_attack_rate: u8,

    /// The rate of decay while sustaining. Between sustain_level and zero.
    pub decay_sustain_rate: u8,

    /// The rate of decay after the key is released
    pub release_rate: u8,
}

impl Default for EnvelopeDefinition {
    fn default() -> Self {
        Self {
            total_level: 0,
            sustain_level: u8::MAX,

            attack_rate: u8::MAX,
            decay_attack_rate: 0,
            decay_sustain_rate: 0,
            release_rate: u8::MAX,
        }
    }
}
