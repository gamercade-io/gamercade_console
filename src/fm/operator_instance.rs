use crate::{
    FMWaveform, OperatorDefinition, OperatorDefinitionBundle, Oscillator, FM_OUTPUT_SAMPLE_RATE,
    LUT_FULL_LEN, OPERATOR_COUNT,
};

#[derive(Debug, Clone)]
pub struct OperatorInstance {
    index: usize,
    oscillator: Oscillator,
}

impl OperatorInstance {
    /// Constructs a new operator instance based on the passed in definition
    pub fn new(index: usize, source: &OperatorDefinition) -> Self {
        Self {
            index,
            oscillator: Oscillator::new(LUT_FULL_LEN),
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator
            .set_frequency(frequency, FM_OUTPUT_SAMPLE_RATE);
    }

    /// Get's the current sample value including any modulation and
    /// interpolates between the next sample.
    /// Also ticks the operator.
    pub fn tick(&mut self, waveform: FMWaveform, modulation: f32) -> f32 {
        let index = self.oscillator.tick() + modulation;

        let next_weight = index.fract();
        let index_weight = 1.0 - next_weight;

        let index = index as usize % LUT_FULL_LEN;
        let next = (index + 1) % LUT_FULL_LEN;

        let index = waveform.lookup(index);
        let next = waveform.lookup(next);

        (index * index_weight) + (next * next_weight)
    }
}

#[derive(Clone)]
pub struct OperatorInstanceBundle {
    pub operators: [OperatorInstance; OPERATOR_COUNT],
}

impl OperatorInstanceBundle {
    pub fn new(source: &OperatorDefinitionBundle) -> Self {
        Self {
            operators: source
                .operators
                .iter()
                .enumerate()
                .map(|(index, operator)| OperatorInstance::new(index, operator))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
