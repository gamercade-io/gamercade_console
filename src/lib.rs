mod envelope;
mod fm;
mod notes;
mod oscillator;
mod sound_engine;
mod sound_rom;
mod tracker;
mod wavetable;

pub use envelope::*;
pub use fm::*;
pub use notes::*;
pub use oscillator::*;
pub use sound_engine::*;
pub use sound_rom::*;
pub use tracker::*;
pub use wavetable::*;

/// Initializes all lookup tables necessary to run.
/// This must be called before calling any other audio related functions.
pub fn initialize_luts() {
    fm::init_fm_lut();
    notes::initialize_notes();
}
