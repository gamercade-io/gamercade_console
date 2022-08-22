mod fm;
mod instrument_data_definition;
mod instrument_instance;
mod sampler;
mod wavetable;
mod wavetable_oscillator;

pub use fm::*;
pub use instrument_data_definition::*;
pub use instrument_instance::*;
pub use sampler::*;
pub use wavetable::*;
pub use wavetable_oscillator::*;

/// The Trigger or Key state for the sound source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Off,
    On,
    Trigger,
}
