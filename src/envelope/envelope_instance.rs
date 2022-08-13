use crate::{ActiveState, EnvelopeDefinition, EnvelopePhase, ExponentialRamp};

#[derive(Clone, Debug)]
pub struct EnvelopeInstance {
    definition: EnvelopeDefinition,
    ramp: ExponentialRamp,
    state: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn new(definition: &EnvelopeDefinition, sample_rate: usize) -> Self {
        Self {
            definition: definition.clone(),
            ramp: ExponentialRamp::new(sample_rate),
            state: EnvelopePhase::Off,
        }
    }

    pub fn tick(&mut self, active: ActiveState) -> f32 {
        if self.definition.total_level == 0 {
            0.0
        } else if ActiveState::Trigger == active {
            self.state = EnvelopePhase::Attack;
            self.ramp
                .set_from_envelope(EnvelopePhase::Attack, &self.definition);
            self.ramp.tick()
        } else {
            match self.state {
                EnvelopePhase::Off => 0.0,
                EnvelopePhase::Attack | EnvelopePhase::Decay | EnvelopePhase::Release => {
                    let out = self.ramp.tick();

                    if self.ramp.is_finished() {
                        self.state = self.state.next_phase();
                        self.ramp.set_from_envelope(self.state, &self.definition)
                    }

                    out
                }
                EnvelopePhase::Sustain => {
                    if ActiveState::Off == active {
                        self.state = self.state.next_phase();
                        self.ramp.set_from_envelope(self.state, &self.definition);
                    }

                    self.ramp.tick()
                }
            }
        }
    }
}
