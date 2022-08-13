mod fm;
mod instrument_definition;
mod wavetable;

pub use fm::*;
pub use instrument_definition::*;
pub use wavetable::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Wavetable,
    FMSynth,
}
