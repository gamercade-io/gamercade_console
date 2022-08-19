use super::SampleDefinition;

#[derive(Debug, Clone)]
pub struct SampleOscillator {
    // Related to the sample itself
    sample_frequency: Option<f32>,
    input_sample_rate: usize,
    pub output_sample_rate: usize,

    // For sound playback
    index: f32,
    index_increment: f32,
    table_length: f32,
}

impl SampleOscillator {
    /// Generates a new SampleOscillator with the default value.
    pub(crate) fn new(output_sample_rate: usize) -> Self {
        Self {
            sample_frequency: None,
            input_sample_rate: output_sample_rate,
            output_sample_rate,
            index: 0.0,
            index_increment: 0.0,
            table_length: 1.0,
        }
    }

    pub(crate) fn from_definition(
        definition: &SampleDefinition,
        output_sample_rate: usize,
    ) -> Self {
        let mut out = Self::new(output_sample_rate);
        out.set_sample(definition);
        out
    }

    /// Sets the frequency of the oscillator. If passed
    /// a None, it will revert the play rate back to the default one of the sample.
    pub(crate) fn set_frequency(&mut self, frequency: Option<f32>) {
        // TODO!
        todo!()
        // let increment = frequency * self.table_length;
        // self.index_increment = increment / self.output_sample_rate as f32;
    }

    /// Sets this oscillator to match the requirements of the passed
    /// in sample.
    pub(crate) fn set_sample(&mut self, sample: &SampleDefinition) {
        self.sample_frequency = sample.sample_frequency;
        self.input_sample_rate = sample.source_sample_rate;
        self.index = 0.0;
        self.index_increment = sample.source_sample_rate as f32 / self.output_sample_rate as f32;
        self.table_length = sample.data.len() as f32;
    }

    /// Returns the index, then
    /// Increments the oscillator by its predefined amount
    pub(crate) fn tick(&mut self) -> f32 {
        let out = self.index;
        self.index += self.index_increment;
        self.index %= self.table_length;
        out
    }
}
