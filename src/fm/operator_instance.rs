use crate::{
    EnvelopeInstance, FMWaveform, OperatorDefinition, OperatorDefinitionBundle, Oscillator,
    FM_OUTPUT_SAMPLE_RATE, LUT_FULL_LEN, OPERATOR_COUNT,
};

#[derive(Debug, Clone)]
pub struct OperatorInstance {
    oscillator: Oscillator,
    envelope: EnvelopeInstance,
}

impl OperatorInstance {
    /// Constructs a new operator instance based on the passed in definition
    pub fn new(source: &OperatorDefinition) -> Self {
        Self {
            oscillator: Oscillator::new(LUT_FULL_LEN),
            envelope: EnvelopeInstance::new(&source.envlope_definition, FM_OUTPUT_SAMPLE_RATE),
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
    pub fn tick(&mut self, waveform: FMWaveform, modulation: f32, active: bool) -> f32 {
        let index = self.oscillator.tick() + modulation;

        let next_weight = index.fract();
        let index_weight = 1.0 - next_weight;

        let index = index as usize % LUT_FULL_LEN;
        let next = (index + 1) % LUT_FULL_LEN;

        let index = waveform.lookup(index);
        let next = waveform.lookup(next);

        let output = (index * index_weight) + (next * next_weight);
        let envelope = self.envelope.tick(active);
        output * envelope
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
                .map(OperatorInstance::new)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
