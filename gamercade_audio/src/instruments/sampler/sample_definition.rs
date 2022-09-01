use serde::{Deserialize, Serialize};

use crate::{EnvelopeDefinition, IndexInterpolator, SampleBitDepth};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDefinition {
    pub data: Box<[SampleBitDepth]>,
    pub source_sample_rate: usize,
    pub sample_frequency: Option<f32>,
    pub envelope_definition: EnvelopeDefinition,
    pub interpolator: IndexInterpolator,
}

impl Default for SampleDefinition {
    fn default() -> Self {
        Self {
            data: vec![0].into_boxed_slice(),
            source_sample_rate: 1,
            sample_frequency: Default::default(),
            envelope_definition: Default::default(),
            interpolator: IndexInterpolator::default(),
        }
    }
}

impl SampleDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
