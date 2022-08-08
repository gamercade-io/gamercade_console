use std::sync::Arc;

use rodio::OutputStream;

use gamercade_audio::{
    EnvelopeDefinition, Waveform, WavetableDefinition, WavetableGenerator, WavetableOscilator,
};

fn main() {
    let data = WavetableGenerator {
        waveform: Waveform::Sine,
        size: 64,
    }
    .generate();

    let def = WavetableDefinition {
        data,
        envelope: Arc::new(EnvelopeDefinition::default()),
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
