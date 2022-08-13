mod fm;
mod instrument_definition;
mod wavetable;

pub use fm::*;
pub use instrument_definition::*;
pub use wavetable::*;

/// A general type to identify various states of instruments by
/// their main kind.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Wavetable,
    FMSynth,
}
