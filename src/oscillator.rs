#[derive(Debug, Clone)]
pub struct Oscillator {
    index: f32,
    index_increment: f32,
    phase_length: f32,
}

impl Oscillator {
    pub(crate) fn new(phase_length: usize) -> Self {
        Self {
            index: 0.0,
            index_increment: 0.0,
            phase_length: phase_length as f32,
        }
    }

    /// Sets the frequency of the oscillator
    pub(crate) fn set_frequency(&mut self, frequency: f32, output_sample_rate: usize) {
        self.index_increment = (frequency * self.phase_length) / output_sample_rate as f32;
    }

    /// Returns the index, then
    /// Increments the oscillator by its predefined amount
    pub(crate) fn tick(&mut self) -> f32 {
        let out = self.index;
        self.index += self.index_increment;
        self.index %= self.phase_length;
        out
    }
}
