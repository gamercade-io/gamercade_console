use std::sync::Arc;

use crate::{EnvelopeDefinition, EnvelopePhase};

#[derive(Clone)]
pub struct EnvelopeInstance {
    definition: Arc<EnvelopeDefinition>,
    state: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn tick(&mut self) -> f32 {
        self.state.tick(&self.definition)
    }
}
