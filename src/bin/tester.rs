use std::sync::Arc;

use gamercade_audio::{Waveform, WavetableGenerator, WavetableOscilator};
use rodio::OutputStream;

fn main() {
    let def = Arc::new(
        WavetableGenerator {
            waveform: Waveform::Saw,
            size: 1280,
        }
        .generate(),
    );

    let mut oscilator = WavetableOscilator::new(22_050, def.clone());
    oscilator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let result = stream_handle.play_raw(oscilator).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
