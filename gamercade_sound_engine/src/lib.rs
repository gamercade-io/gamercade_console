mod envelope;
mod instruments;
mod playback;
mod sound_engine;
mod sound_output_channels;
mod sound_rom_instance;

pub use envelope::*;
pub use instruments::*;
pub use playback::*;
pub use sound_engine::*;
pub use sound_output_channels::*;
pub use sound_rom_instance::*;

use gamercade_audio::{EnvelopeDefinition, WavetableDefinition};

fn initialize_globals() {
    init_fm_lut();
    gamercade_audio::initialize_notes();
    unsafe {
        NO_SOUND_DEFINITION.write(std::sync::Arc::new(WavetableDefinition {
            data: Box::new([0, 0]),
            envelope: EnvelopeDefinition::default(),
            interpolator: gamercade_audio::IndexInterpolator::Truncate,
        }));
    }
}
