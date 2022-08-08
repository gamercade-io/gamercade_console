use std::sync::Arc;

use super::{Algorithm, FeedbackLevel, OperatorDefinition, OPERATOR_COUNT};

pub struct PatchDefinition {
    pub operators: Arc<[OperatorDefinition; OPERATOR_COUNT]>,
    pub algorithm: Algorithm,
    pub feedback: FeedbackLevel,
}
