use std::{mem::MaybeUninit, sync::Arc};

use gamercade_audio::{WavetableBitDepth, WavetableDefinition};

use crate::{ActiveState, EnvelopeInstance, WavetableOscillator};

pub(crate) static mut NO_SOUND_DEFINITION: MaybeUninit<Arc<WavetableDefinition>> =
    MaybeUninit::uninit();

#[derive(Clone, Debug)]
pub struct WavetableInstance {
    definition: Arc<WavetableDefinition>,
    envelope: EnvelopeInstance,
    pub(crate) oscillator: WavetableOscillator,
    active: ActiveState,
}

impl WavetableInstance {
    pub fn no_sound(output_sample_rate: usize) -> Self {
        let definition = unsafe { NO_SOUND_DEFINITION.assume_init_ref().clone() };
        Self {
            envelope: EnvelopeInstance::no_sound(output_sample_rate),
            definition,
            oscillator: WavetableOscillator::new(1, output_sample_rate),
            active: ActiveState::Off,
        }
    }

    /// Generates a new WavetableOscilator
    pub fn new(definition: Arc<WavetableDefinition>, output_sample_rate: usize) -> Self {
        Self {
            envelope: EnvelopeInstance::new(&definition.envelope, output_sample_rate),
            oscillator: WavetableOscillator::new(definition.len(), output_sample_rate),
            definition,
            active: ActiveState::Off,
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
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
