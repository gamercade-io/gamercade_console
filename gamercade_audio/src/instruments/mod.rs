mod fm;
mod index_interpolator;
mod instrument_data_definition;
mod sampler;
mod wavetable;

pub use fm::*;
pub use index_interpolator::*;
pub use instrument_data_definition::*;
pub use sampler::*;
pub use wavetable::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Sampler,
    FMSynth,
    Wavetable,
}
