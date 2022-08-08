use std::sync::Arc;

use rodio::Source;

use crate::OUTPUT_SAMPLE_RATE;

use super::{WavetableBitDepth, WavetableDefinition};

pub struct WavetableOscilator {
    definition: Arc<WavetableDefinition>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscilator {
    /// Generates a new WavetableOscilator
    pub fn new(definition: Arc<WavetableDefinition>) -> Self {
        Self {
            definition,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment =
            (frequency * self.definition.len() as f32) / OUTPUT_SAMPLE_RATE as f32;
    }

    /// Get's the current sample value
    /// This interpolates between the current index and the next index
    pub fn get_sample(&self) -> f32 {
        let index = self.index as usize;
        let next = (index + 1) % self.definition.len();

        let next_weight = self.index.fract();
        let index_weight = 1.0 - next_weight;

        let index = self.definition.data[index] as f32 / WavetableBitDepth::MAX as f32;
        let next = self.definition.data[next] as f32 / WavetableBitDepth::MAX as f32;

        (index * index_weight) + (next * next_weight)
    }

    /// Increments the oscilator by its predefined amounts
    pub fn increment(&mut self) {
        self.index += self.index_increment;
        self.index %= self.definition.len() as f32;
    }
}

impl Iterator for WavetableOscilator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let output = self.get_sample() * 0.1;
        self.increment();
        Some(output)
    }
}

impl Source for WavetableOscilator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        OUTPUT_SAMPLE_RATE as u32
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
