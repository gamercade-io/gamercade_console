mod envelope;
mod fm;
mod waveform;
mod wavetable;

pub use envelope::*;
pub use waveform::*;
pub use wavetable::*;

pub const OUTPUT_SAMPLE_RATE: usize = 44_100; // 44.1 khz
