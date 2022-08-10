use crate::{EnvelopeDefinition, EnvelopePhase, Ramp};

#[derive(Clone, Debug)]
pub struct EnvelopeInstance {
    definition: EnvelopeDefinition,
    ramp: Ramp,
    state: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn new(definition: &EnvelopeDefinition, sample_rate: usize) -> Self {
        Self {
            definition: definition.clone(),
            ramp: Ramp::new(sample_rate),
            state: EnvelopePhase::Off,
        }
    }

    pub fn tick(&mut self, active: bool) -> f32 {
        if self.definition.total_level == 0 {
            0.0
        } else {
            match self.state {
                EnvelopePhase::Off => {
                    if active {
                        self.state = EnvelopePhase::Attack;
                        self.ramp
                            .generate_from_definition(EnvelopePhase::Attack, &self.definition);
                        self.ramp.tick()
                    } else {
                        0.0
                    }
                }
                EnvelopePhase::Attack | EnvelopePhase::Decay => {
                    let out = self.ramp.tick();

                    if self.ramp.is_finished() {
                        self.state = self.state.next_phase();
                        self.ramp
                            .generate_from_definition(self.state, &self.definition)
                    }

                    out
                }
                EnvelopePhase::Sustain => {
                    if !active {
                        self.state = self.state.next_phase();
                        self.ramp
                            .generate_from_definition(self.state, &self.definition);
                    }

                    self.ramp.tick()
                }
                EnvelopePhase::Release => {
                    if active {
                        self.state = EnvelopePhase::Attack;
                        self.ramp
                            .generate_from_definition(EnvelopePhase::Attack, &self.definition);
                    }
                    self.ramp.tick()
                }
            }
        }
    }
}
