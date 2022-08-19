use std::sync::Arc;

use crate::{ActiveState, EnvelopeInstance, SampleBitDepth, SampleDefinition, SampleOscillator};

#[derive(Debug, Clone)]
pub struct SamplerInstance {
    pub oscillator: SampleOscillator,
    definition: Arc<SampleDefinition>,
    active: ActiveState,
    envelope: EnvelopeInstance,
}

impl SamplerInstance {
    pub fn new(definition: &Arc<SampleDefinition>, output_sample_rate: usize) -> Self {
        Self {
            oscillator: SampleOscillator::from_definition(definition, output_sample_rate),
            definition: definition.clone(),
            active: ActiveState::Off,
            envelope: EnvelopeInstance::new(&definition.envelope_definition, output_sample_rate),
        }
    }
    /// Get's the current sample value
    /// This interpolates between the current index and the next index
    /// Also increments the oscillator
    pub fn tick(&mut self) -> f32 {
        let index = self.oscillator.tick();

        let next_weight = index.fract();
        let index_weight = 1.0 - next_weight;

        let index = index as usize;
        let next = (index + 1) % self.definition.len();

        let index = self.definition.data[index] as f32 / SampleBitDepth::MAX as f32;
        let next = self.definition.data[next] as f32 / SampleBitDepth::MAX as f32;

        let output = (index * index_weight) + (next * next_weight);

        let envelope = self.envelope.tick(self.active);

        if ActiveState::Trigger == self.active {
            self.active = ActiveState::Off;
        }

        output * envelope
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(Some(frequency))
    }

    pub fn set_active(&mut self, active: bool) {
        if active {
            self.active = ActiveState::On
        } else {
            self.active = ActiveState::Off
        }
    }

    pub fn trigger(&mut self) {
        self.active = ActiveState::Trigger
    }
}
