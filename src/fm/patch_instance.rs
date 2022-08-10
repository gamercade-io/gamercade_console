use std::sync::Arc;

use crate::{OperatorInstanceBundle, PatchDefinition};

#[derive(Clone)]
pub struct PatchInstance {
    operators: OperatorInstanceBundle,
    definition: Arc<PatchDefinition>,
    active: bool,
    feedback: [f32; 2],
}

impl PatchInstance {
    pub fn new(definition: Arc<PatchDefinition>) -> Self {
        Self {
            operators: OperatorInstanceBundle::new(&definition.operators),
            definition,
            active: false,
            feedback: [0.0; 2],
        }
    }
}
