/// A wavetable oscillator. Returns table indices.
#[derive(Debug, Clone)]
pub struct WavetableOscillator {
    index: f32,
    index_increment: f32,
    table_length: f32,
    pub(crate) output_sample_rate: usize,
}

/// The Trigger or Key state for the sound source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Off,
    On,
    Trigger,
}

impl WavetableOscillator {
    /// Generates a new WavetableOscillator with the default value.
    pub(crate) fn new(table_length: usize, output_sample_rate: usize) -> Self {
        Self {
            index: 0.0,
            index_increment: 0.0,
            table_length: table_length as f32,
            output_sample_rate,
        }
    }

    /// Sets the frequency of the oscillator
    pub(crate) fn set_frequency(&mut self, frequency: f32) {
        let increment = frequency * self.table_length;
        self.index_increment = increment / self.output_sample_rate as f32;
    }

    /// Returns the modulation amount for this oscillator. Used with FM Synth
    pub(crate) fn modulation(&self, modulation: f32) -> f32 {
        modulation * self.index_increment
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
