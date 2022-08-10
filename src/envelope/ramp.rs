use std::ops::Sub;

use crate::{EnvelopeDefinition, EnvelopePhase};

const OVERSHOOT: f32 = 1.001;

#[derive(Clone, Debug)]
pub(crate) struct Ramp {
    sample_rate: usize,
    value: f32,              // The current value
    target_value: f32,       // The "end" value
    overshoot_value: f32,    // The "overshoot" value since we are dealing a small margin of error
    decaying_increment: f32, // The increment which changes over time
    multiplier: f32,         // The multiplier for the increment
}

impl Ramp {
    pub fn new<T>(sample_rate: usize, start_value: T, target_value: T, time: f32) -> Self
    where
        f32: From<<T as Sub>::Output> + From<T>,
        T: Sub + Copy,
    {
        let mut out = Self {
            sample_rate,
            value: start_value.into(),
            target_value: 0.0,
            overshoot_value: 0.0,
            decaying_increment: 0.0,
            multiplier: 0.0,
        };

        out.ramp_to(target_value.into(), time);

        out
    }

    pub fn generate_from_definition(
        &mut self,
        phase: EnvelopePhase,
        definition: &EnvelopeDefinition,
    ) {
        match phase {
            EnvelopePhase::Attack => todo!(),
            EnvelopePhase::Decay => todo!(),
            EnvelopePhase::Sustain => todo!(),
            EnvelopePhase::Release => todo!(),
            EnvelopePhase::Off => self.set_constant_value(0.0),
        }
    }

    // Causes the ramp to hold at the passed in value
    fn set_constant_value(&mut self, new_value: f32) {
        self.value = new_value;
        self.target_value = new_value;
        self.overshoot_value = new_value;
        self.decaying_increment = 0.0;
        self.multiplier = 0.0;
    }

    fn ramp_to(&mut self, target_value: f32, time: f32) {
        self.target_value = target_value;

        let distance_to_target = target_value - self.value;
        self.overshoot_value = self.value + (distance_to_target * OVERSHOOT);

        self.multiplier = self.value - self.overshoot_value;

        let time = -1.0 * time / (1.0 - (1.0 / OVERSHOOT)).log10();
        self.multiplier = f32::powf(f32::exp(-1.0 / time), 1.0 / self.sample_rate as f32);
    }

    pub fn tick(&mut self) -> f32 {
        self.value = self.overshoot_value + self.decaying_increment;

        if !self.is_finished() {
            self.decaying_increment *= self.multiplier;
        }

        self.value
    }

    pub fn is_finished(&self) -> bool {
        if self.value >= self.target_value && self.value <= self.overshoot_value {
            true
        } else if self.value <= self.target_value && self.value >= self.overshoot_value {
            true
        } else {
            false
        }
    }
}
