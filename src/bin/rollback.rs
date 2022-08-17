use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use gamercade_audio::{
    EnvelopeDefinition, WavetableDefinition, WavetableGenerator, WavetableOscilator,
    WavetableWaveform,
};

use rodio::{
    cpal::{
        self, default_host,
        traits::{HostTrait, StreamTrait},
        StreamConfig,
    },
    DeviceTrait,
};
use rtrb::RingBuffer;

const SAMPLE_RATE: u32 = 48_000; // hard coded to my system 
const FPS: u32 = 30;
const BUFFER_LENGTH: usize = (SAMPLE_RATE / FPS) as usize;

pub fn main() {
    let device = default_host().default_output_device().unwrap();

    let supported_config = device
        .supported_output_configs()
        .unwrap()
        .next()
        .unwrap()
        .with_max_sample_rate();
    println!("sample rate: {:?}", supported_config.sample_rate());
    let config = StreamConfig::from(supported_config);

    let (mut producer, mut consumer) = RingBuffer::new(BUFFER_LENGTH * 3); // enough to store 3 game frames

    let mut osci = osci();
    osci.set_frequency(400.0);
    osci.trigger();

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
                data.chunks_exact_mut(2)
                    .for_each(|frame| match consumer.pop() {
                        Ok(next) => {
                            frame[0] = next;
                            frame[1] = next;
                        }
                        Err(_) => println!("audio thread starved"),
                    });
            },
            move |err| {
                // react to errors here.
                println!("{}", err);
            },
        )
        .unwrap();

    #[derive(PartialEq, Eq)]
    enum WriterState {
        Writing,
        FrameComplete,
    }

    std::thread::spawn(move || {
        let mut audio_frames = 0;
        let mut state = WriterState::Writing;
        let mut prev = Instant::now();
        loop {
            if producer.is_full() || state == WriterState::FrameComplete {
                //print!("full or done");
            } else {
                producer.push(osci.tick()).unwrap();
                audio_frames += 1;
                if audio_frames == BUFFER_LENGTH {
                    println!("wrote a frame, save state");
                    audio_frames = 0;
                    state = WriterState::FrameComplete;
                }
            }

            if Instant::now().duration_since(prev) >= Duration::from_secs_f32(1.0 / FPS as f32) {
                state = WriterState::Writing;
                prev = Instant::now();
            }
        }
    });

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(5));
}

fn osci() -> WavetableOscilator {
    WavetableOscilator::new(Arc::new(WavetableDefinition {
        data: WavetableGenerator {
            waveform: WavetableWaveform::Sine,
            size: 64,
        }
        .generate(),
        envelope: EnvelopeDefinition::interesting(),
        sample_rate: SAMPLE_RATE as usize,
    }))
}
