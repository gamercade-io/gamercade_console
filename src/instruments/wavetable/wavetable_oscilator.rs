use std::{mem::MaybeUninit, sync::Arc};

use rodio::Source;

use crate::{ActiveState, EnvelopeInstance, Oscillator, WavetableBitDepth};

use super::WavetableDefinition;

pub(crate) static mut NO_SOUND_DEFINITION: MaybeUninit<Arc<WavetableDefinition>> =
    MaybeUninit::uninit();

pub(crate) const NO_SOUND_SAMPLE_RATE: usize = 11_025; //44_100Khz / 4

#[derive(Clone)]
pub struct WavetableOscilator {
    definition: Arc<WavetableDefinition>,
    envelope: EnvelopeInstance,
    oscillator: Oscillator,
    active: ActiveState,
}

impl WavetableOscilator {
    pub fn no_sound() -> Self {
        let definition = unsafe { NO_SOUND_DEFINITION.assume_init_ref().clone() };
        Self {
            envelope: EnvelopeInstance::no_sound(),
            definition,
            oscillator: Oscillator::new(1),
            active: ActiveState::Off,
        }
    }

    /// Generates a new WavetableOscilator
    pub fn new(definition: Arc<WavetableDefinition>) -> Self {
        Self {
            envelope: EnvelopeInstance::new(&definition.envelope, definition.sample_rate),
            oscillator: Oscillator::new(definition.len()),
            definition,
            active: ActiveState::Off,
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

        let output = (index * index_weight) + (next * next_weight);
        let envelope = self.envelope.tick(self.active);

        if ActiveState::Trigger == self.active {
            self.active = ActiveState::Off;
        }

        output * envelope
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = if active {
            ActiveState::On
        } else {
            ActiveState::Off
        };
    }

    pub fn trigger(&mut self) {
        self.active = ActiveState::Trigger;
    }
}

impl Iterator for WavetableOscilator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.tick())
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
