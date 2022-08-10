use std::sync::Arc;

use rodio::Source;

use crate::{Oscillator, WavetableBitDepth};

use super::WavetableDefinition;

pub struct WavetableOscilator {
    definition: Arc<WavetableDefinition>,
    oscillator: Oscillator,
    active: bool,
}

impl WavetableOscilator {
    /// Generates a new WavetableOscilator
    pub fn new(definition: Arc<WavetableDefinition>) -> Self {
        Self {
            oscillator: Oscillator::new(definition.len()),
            definition,
            active: false,
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator
            .set_frequency(frequency, self.definition.sample_rate);
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

        let index = self.definition.data[index] as f32 / WavetableBitDepth::MAX as f32;
        let next = self.definition.data[next] as f32 / WavetableBitDepth::MAX as f32;

        (index * index_weight) + (next * next_weight)
    }
}

impl Iterator for WavetableOscilator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let output = self.tick() * 0.15;
        Some(output)
    }
}

impl Source for WavetableOscilator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.definition.sample_rate as u32
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
