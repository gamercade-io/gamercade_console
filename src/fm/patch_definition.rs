use serde::{Deserialize, Serialize};

use super::{Algorithm, FeedbackLevel, OperatorDefinition, OPERATOR_COUNT};

#[derive(Clone, Serialize, Deserialize)]
pub struct PatchDefinition {
    pub operators: [OperatorDefinition; OPERATOR_COUNT],
    pub algorithm: Algorithm,
    pub feedback: FeedbackLevel,
}
