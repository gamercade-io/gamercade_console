use std::sync::Arc;

use rodio::Source;

use crate::{
    ModulatedBy, OperatorInstanceBundle, PatchDefinition, FM_AMPLIFICATION, OPERATOR_COUNT,
};

#[derive(Clone)]
pub struct PatchInstance {
    operators: OperatorInstanceBundle,
    definition: Arc<PatchDefinition>,
    feedback: [f32; 2],
    active: bool,
}

impl PatchInstance {
    pub fn new(definition: Arc<PatchDefinition>) -> Self {
        Self {
            operators: OperatorInstanceBundle::new(&definition.operators),
            definition,
            feedback: [0.0; 2],
            active: false,
        }
    }

    /// Sets the base frequency of the entire patch
    pub fn set_frequency(&mut self, frequency: f32) {
        let instances = self.operators.operators.iter_mut();
        let definitions = self.definition.operators.operators.iter();
        instances
            .zip(definitions)
            .for_each(|(instance, definition)| {
                let adjusted_frequency = definition.frequency_multiplier.multiply(frequency);
                instance.set_frequency(adjusted_frequency)
            });
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn tick(&mut self) -> f32 {
        let mut outputs = [0.0f32; OPERATOR_COUNT];
        let mut final_output = 0.0f32;

        let algorithm = self.definition.algorithm.get_definition();
        let operators = &mut self.operators.operators;
        let operator_definitions = &self.definition.operators.operators;

        // 1st Operator is always feedback
        let feedback_input = ((self.feedback[0] + self.feedback[1]) / 2.0)
            * self.definition.feedback.as_multiplier();

        outputs[0] = operators[0].tick(
            operator_definitions[0].waveform,
            feedback_input,
            self.active,
        );

        // Handle feedback
        self.feedback[1] = self.feedback[0];
        self.feedback[0] = outputs[1];

        if algorithm.carriers[0] {
            final_output += outputs[0] * FM_AMPLIFICATION;
        }
        // End 1st Operator

        // Handle the rest of the operators
        (1..OPERATOR_COUNT).for_each(|i| {
            let operator = &mut operators[i];
            let waveform = operator_definitions[i].waveform;
            let modulator = &algorithm.modulators[i - 1];

            let modulation = match modulator {
                ModulatedBy::None => 0.0,
                ModulatedBy::Single(modulator) => outputs[*modulator],
                ModulatedBy::Double(first, second) => outputs[*first] + outputs[*second],
                ModulatedBy::Triple(first, second, third) => {
                    outputs[*first] + outputs[*second] + outputs[*third]
                }
            };

            let result = operator.tick(waveform, modulation, self.active);

            let result = result * FM_AMPLIFICATION;

            outputs[i] = result;

            if algorithm.carriers[i] {
                final_output += result;
            }
        });

        final_output / FM_AMPLIFICATION
    }
}

impl Iterator for PatchInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}

impl Source for PatchInstance {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        crate::FM_OUTPUT_SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
