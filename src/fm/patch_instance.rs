use std::sync::Arc;

use rodio::Source;

use crate::{
    ModulatedBy, OperatorInstanceBundle, PatchDefinition, FM_AMPLIFICATION, OPERATOR_COUNT,
};

#[derive(Clone)]
pub struct PatchInstance {
    operators: OperatorInstanceBundle,
    definition: Arc<PatchDefinition>,
    active: bool,
    feedback: [f32; 2],
}

impl PatchInstance {
    pub fn new(definition: Arc<PatchDefinition>) -> Self {
        Self {
            operators: OperatorInstanceBundle::new(&definition.operators),
            definition,
            active: false,
            feedback: [0.0; 2],
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
}

impl Iterator for PatchInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut outputs = [0.0f32; OPERATOR_COUNT];
        let mut final_output = 0.0f32;

        let algorithm = self.definition.algorithm.get_definition();
        let operators = &mut self.operators.operators;
        let operator_definitions = &self.definition.operators.operators;

        // 1st Operator is always feedback
        let feedback_input = ((self.feedback[0] + self.feedback[1]) / 2.0)
            * self.definition.feedback.as_multiplier();
        outputs[0] = operators[0].get_sample(operator_definitions[0].waveform, feedback_input);
        operators[0].tick();

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

            // TODO: Remove this when modulation is working
            let result = operator.get_sample(waveform, 0.0);

            // let result = match modulator {
            //     ModulatedBy::None => operator.get_sample(waveform, 0.0),
            //     ModulatedBy::Single(modulator) => {
            //         operator.get_sample(waveform, outputs[*modulator])
            //     }
            //     ModulatedBy::Double(first, second) => {
            //         operator.get_sample(waveform, outputs[*first] + outputs[*second])
            //     }
            //     ModulatedBy::Triple(first, second, third) => operator.get_sample(
            //         waveform,
            //         outputs[*first] + outputs[*second] + outputs[*third],
            //     ),
            // };

            operator.tick();

            let result = result * FM_AMPLIFICATION;

            outputs[i] = result;

            if algorithm.carriers[i] {
                final_output += result;
            }
        });

        Some(final_output / FM_AMPLIFICATION)
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
