use serde::{Deserialize, Serialize};

use crate::{EnvelopeDefinition, SampleBitDepth};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDefinition {
    pub data: Box<[SampleBitDepth]>,
    pub source_sample_rate: usize,
    pub sample_frequency: Option<f32>,
    pub envelope_definition: EnvelopeDefinition,
}

impl SampleDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
