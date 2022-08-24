mod consts;
mod envelope;
mod instruments;
mod notes;
mod sound_rom;
mod tracker;

pub use consts::*;
pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use sound_rom::*;
pub use tracker::*;

#[cfg(feature = "playback")]
mod playback;
#[cfg(feature = "playback")]
mod sound_engine;
#[cfg(feature = "playback")]
mod sound_rom_instance;

#[cfg(feature = "playback")]
pub use playback::*;
#[cfg(feature = "playback")]
pub use sound_engine::*;
#[cfg(feature = "playback")]
pub use sound_rom_instance::*;

#[cfg(feature = "playback")]
pub fn initialize_globals() {
    init_fm_lut();
    initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
        }));
    }
}
