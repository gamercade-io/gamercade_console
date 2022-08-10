use serde::{Deserialize, Serialize};

use crate::OperatorDefinitionBundle;

use super::{Algorithm, FeedbackLevel, OperatorDefinition, OPERATOR_COUNT};

#[derive(Clone, Serialize, Deserialize)]
pub struct PatchDefinition {
    pub operators: OperatorDefinitionBundle,
    pub algorithm: Algorithm,
    pub feedback: FeedbackLevel,
}
