use serde::{Deserialize, Serialize};

use super::WavetableBitDepth;
use crate::EnvelopeDefinition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavetableDefinition {
    pub data: Box<[WavetableBitDepth]>,
    pub envelope: EnvelopeDefinition,
}

impl Default for WavetableDefinition {
    fn default() -> Self {
        Self {
            data: vec![0].into_boxed_slice(),
            envelope: Default::default(),
        }
    }
}

impl WavetableDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
