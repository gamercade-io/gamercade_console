use crate::{EnvelopeDefinition, Ramp};

#[derive(Clone, Debug)]
pub(crate) enum EnvelopePhase {
    Attack(Ramp),
    Decay(Ramp),
    Sustain(Ramp),
    Release(Ramp),
    Off,
}

impl EnvelopePhase {
    pub fn tick(&mut self, definition: &EnvelopeDefinition) -> f32 {
        match self {
            EnvelopePhase::Attack(ramp) => {
                if let Some(out) = ramp.next() {
                    out
                } else {
                    let (next, out) = definition.generate_decay();
                    *self = next;
                    out
                }
            }
            EnvelopePhase::Decay(ramp) => {
                if let Some(out) = ramp.next() {
                    out
                } else {
                    let (next, out) = definition.generate_sustain();
                    *self = next;
                    out
                }
            }
            EnvelopePhase::Sustain(ramp) | EnvelopePhase::Release(ramp) => {
                if let Some(out) = ramp.next() {
                    out
                } else {
                    *self = EnvelopePhase::Off;
                    0.0
                }
            }
            EnvelopePhase::Off => 0.0,
        }
    }
}
