use std::sync::Arc;

use crate::{OperatorDefinition, Oscillator};

#[derive(Clone)]
pub struct OperatorInstance {
    definition: Arc<OperatorDefinition>,
    oscillator: Oscillator,
}
