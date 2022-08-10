mod envelope_definition;
mod envelope_instance;
mod envelope_phase;
mod ramp;

pub use envelope_definition::*;
pub use envelope_instance::*;
pub(crate) use envelope_phase::*;
pub(crate) use ramp::*;

pub const ENVELOPE_TIME_SCALE: f32 = 256.0; // Max length in seconds, ~4.25mins
pub type EnvelopeType = u16;
