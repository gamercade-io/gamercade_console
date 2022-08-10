mod envelope_definition;
mod envelope_instance;
mod envelope_phase;
mod ramp;

pub use envelope_definition::*;
pub use envelope_instance::*;
pub use envelope_phase::*;
pub use ramp::*;

pub const ENVELOPE_TIME_SCALE: f32 = 1.0; // In seconds
pub type EnvelopeType = u8;
