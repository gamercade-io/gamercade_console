mod envelope_definition;
mod envelope_instance;
mod envelope_phase;
mod exponential_ramp;
mod linear_ramp;

pub use envelope_definition::*;
pub use envelope_instance::*;
pub(crate) use envelope_phase::*;
pub use exponential_ramp::*;
pub use linear_ramp::*;

/// Max length in seconds, ~4.25mins.
pub(crate) const ENVELOPE_TIME_SCALE: f32 = 256.0;

/// The integer type used to store envelopes.
pub type EnvelopeType = u16;
