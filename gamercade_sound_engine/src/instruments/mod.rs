mod fm;
mod instrument_instance;
mod sampler;
mod wavetable;

pub use fm::*;
pub use instrument_instance::*;
pub use sampler::*;
pub use wavetable::*;

/// The Trigger or Key state for the sound source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Off,
    On,
    Trigger,
}
