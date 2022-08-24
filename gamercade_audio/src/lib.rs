mod envelope;
mod instruments;
mod notes;
mod playback;
mod sound_engine;
mod sound_rom;
mod sound_rom_instance;
mod tracker;

pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use sound_rom::*;
pub use tracker::*;

#[cfg(feature = "playback")]
pub use playback::*;
#[cfg(feature = "playback")]
pub use sound_engine::*;
#[cfg(feature = "playback")]
pub use sound_rom_instance::*;

pub const MAX_ROLLBACK_SOUNDS: usize = 8;

#[cfg(feature = "playback")]
fn initialize_globals() {
    init_fm_lut();
    initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
        }));
    }
}
