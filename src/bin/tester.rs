use std::sync::Arc;

use rodio::OutputStream;

use gamercade_audio::{
    init_fm_lut, EnvelopeDefinition, WavetableDefinition, WavetableGenerator, WavetableOscilator,
    WavetableWaveform,
};

fn main() {
    init_fm_lut();

    let data = WavetableGenerator {
        waveform: WavetableWaveform::Sine,
        size: 64,
    }
    .generate();

    let def = WavetableDefinition {
        data,
        envelope: Arc::new(EnvelopeDefinition::default()),
        sample_rate: 44_100, //44.1 khz
    };

    let mut oscilator = WavetableOscilator::new(Arc::new(def));
    // Max: 7902.132820098003
    // Default: 440.0
    // Min: 32.703195662574764
    oscilator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    stream_handle.play_raw(oscilator).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
