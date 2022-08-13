mod envelope;
mod instruments;
mod notes;
mod oscillator;
mod playback;
mod sound_engine;
mod sound_rom;
mod tracker;

pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use oscillator::*;
pub use playback::*;
pub use sound_engine::*;
pub use sound_rom::*;
pub use tracker::*;

/// Initializes all lookup tables necessary to run.
/// This must be called before calling any other audio related functions.
pub fn initialize_luts() {
    init_fm_lut();
    initialize_notes();
}
