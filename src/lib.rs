mod envelope;
mod instruments;
mod notes;
mod oscillator;
mod playback;
mod sound_rom;
mod sound_rom_instance;
mod tracker;

pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use oscillator::*;
pub use playback::*;
pub use sound_rom::*;
pub use sound_rom_instance::*;
pub use tracker::*;

/// Initializes all lookup tables necessary to run.
/// This must be called before calling any other audio related functions.
pub fn initialize_luts() {
    init_fm_lut();
    initialize_notes();
}
