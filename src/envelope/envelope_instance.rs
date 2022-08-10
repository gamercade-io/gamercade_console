use std::sync::Arc;

use crate::{EnvelopeDefinition, EnvelopePhase, Ramp};

#[derive(Clone)]
pub struct EnvelopeInstance {
    definition: Arc<EnvelopeDefinition>,
    ramp: Ramp,
    state: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn tick(&mut self) -> f32 {
        if EnvelopePhase::Off == self.state {
            0.0
        } else {
            let out = self.ramp.tick();

            if self.ramp.is_finished() {
                self.state = self.state.next_phase();
                self.ramp
                    .generate_from_definition(self.state, &self.definition)
            }

            out
        }
    }
}
