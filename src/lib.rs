mod envelope;
mod instruments;
mod notes;
mod oscillator;
mod playback;
mod sound_engine;
mod sound_rom;
mod sound_rom_instance;
mod tracker;

pub use envelope::*;
pub use instruments::*;
pub use notes::*;
pub use oscillator::*;
pub use playback::*;
pub use sound_engine::*;
pub use sound_rom::*;
pub use sound_rom_instance::*;
pub use tracker::*;

fn initialize_globals() {
    init_fm_lut();
    initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
            sample_rate: NO_SOUND_SAMPLE_RATE,
        }));
    }
}
