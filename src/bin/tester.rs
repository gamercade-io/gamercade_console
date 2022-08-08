use std::sync::Arc;

use rodio::OutputStream;

use gamercade_audio::{
    EnvelopeDefinition, Waveform, WavetableDefinition, WavetableGenerator, WavetableOscilator,
};

fn main() {
    let data = WavetableGenerator {
        waveform: Waveform::Saw,
        size: 1280,
    }
    .generate();

    let def = WavetableDefinition {
        data,
        envelope: Arc::new(EnvelopeDefinition {}),
    };

    let mut oscilator = WavetableOscilator::new(Arc::new(def));
    oscilator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    stream_handle.play_raw(oscilator).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
