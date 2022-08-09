use std::sync::Arc;

use crate::{OperatorInstance, PatchDefinition, OPERATOR_COUNT};

#[derive(Clone)]
pub struct PatchInstance {
    operators: [OperatorInstance; OPERATOR_COUNT],
    definition: Arc<PatchDefinition>,
    active: bool,
    feedback: [f32; 2],
}
