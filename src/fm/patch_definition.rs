use serde::{Deserialize, Serialize};

use crate::OperatorDefinitionBundle;

use super::{Algorithm, FeedbackLevel};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchDefinition {
    pub operators: OperatorDefinitionBundle,
    pub algorithm: Algorithm,
    pub feedback: FeedbackLevel,
}
