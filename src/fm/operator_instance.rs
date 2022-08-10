use crate::{OperatorDefinition, OperatorDefinitionBundle, Oscillator, LUT_LEN, OPERATOR_COUNT};

#[derive(Debug, Clone)]
pub struct OperatorInstance {
    index: usize,
    oscillator: Oscillator,
}

impl OperatorInstance {
    pub fn new(index: usize, source: &OperatorDefinition) -> Self {
        Self {
            index,
            oscillator: Oscillator::new(LUT_LEN),
        }
    }
}

#[derive(Clone)]
pub struct OperatorInstanceBundle {
    pub operators: [OperatorInstance; OPERATOR_COUNT],
}

impl OperatorInstanceBundle {
    pub fn new(source: &OperatorDefinitionBundle) -> Self {
        Self {
            operators: source
                .operators
                .iter()
                .enumerate()
                .map(|(index, operator)| OperatorInstance::new(index, operator))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
