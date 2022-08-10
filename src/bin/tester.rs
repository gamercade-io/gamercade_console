use std::sync::Arc;

use rodio::OutputStream;

use gamercade_audio::{
    init_fm_lut, EnvelopeDefinition, PatchDefinition, PatchInstance, WavetableDefinition,
    WavetableGenerator, WavetableOscilator, WavetableWaveform,
};

fn main() {
    // Initialization
    init_fm_lut();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize our sound sources
    let mut wavetable = wavetable_test();
    let mut fm = fm_test();

    // Max: 7902.132820098003
    // Default: 440.0
    // Min: 32.703195662574764
    fm.set_frequency(440.0);
    wavetable.set_frequency(440.0);

    stream_handle.play_raw(fm).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn fm_test() -> PatchInstance {
    let definition = Arc::new(PatchDefinition::default());

    PatchInstance::new(definition)
}

fn wavetable_test() -> WavetableOscilator {
    let data = WavetableGenerator {
        waveform: WavetableWaveform::Sine,
        size: 64,
    }
    .generate();

    let def = WavetableDefinition {
        data,
        envelope: EnvelopeDefinition::default(),
        sample_rate: 44_100, //44.1 khz
    };

    WavetableOscilator::new(Arc::new(def))
}
