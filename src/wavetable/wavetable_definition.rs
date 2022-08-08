use std::sync::Arc;

use super::WavetableBitDepth;
use crate::EnvelopeDefinition;

pub struct WavetableDefinition {
    pub data: Box<[WavetableBitDepth]>,
    pub envelope: Arc<EnvelopeDefinition>,
}

impl WavetableDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
