mod fm;
mod instrument_definition;
mod instrument_instance;
mod sampler;
mod wavetable;
mod wavetable_oscillator;

pub use fm::*;
pub use instrument_definition::*;
pub use instrument_instance::*;
pub use sampler::*;
pub use wavetable::*;
pub use wavetable_oscillator::*;

/// A general type to identify various states of instruments by
/// their main kind.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Wavetable,
    FMSynth,
    Sampler,
}

/// The Trigger or Key state for the sound source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Off,
    On,
    Trigger,
}
