use std::{process, sync::Arc, time::Duration};

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
use rtrb::{Consumer, Producer, RingBuffer};
use spin_sleep::LoopHelper;

const SAMPLE_RATE: u32 = 48_000; // hard coded to my system
const FPS: u32 = 60;
const BUFFER_LENGTH: usize = (SAMPLE_RATE / FPS) as usize;

// enough to store 1 full "game frame" of audio
fn ring_buf<T>() -> (Producer<T>, Consumer<T>) {
    RingBuffer::new(BUFFER_LENGTH)
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

pub fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        process::exit(1);
    }));

    let device = default_host().default_output_device().unwrap();

    let supported_config = device
        .supported_output_configs()
        .unwrap()
        .next()
        .unwrap()
        .with_max_sample_rate();
    println!("sample rate: {:?}", supported_config.sample_rate());
    let config = StreamConfig::from(supported_config);

    // Produces buffers full of "frames"
    let (mut buffer_producer, mut buffer_consumer) = RingBuffer::new(2);
    let (mut producer, mut consumer) = ring_buf();

    // Write silence for testing
    producer
        .write_chunk_uninit(BUFFER_LENGTH)
        .unwrap()
        .fill_from_iter(Some(0.0).iter().cycle().cloned());

    let mut osci = osci();
    osci.set_frequency(400.0);
    osci.trigger();

    let mut frames_read = 0;

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
                data.chunks_exact_mut(2).for_each(|frame| {
                    frames_read += 1;

                    match consumer.pop() {
                        Err(_) => println!("audio inner buffer starved"),
                        Ok(next_sample) => {
                            // Write the samples out
                            frame[0] = next_sample;
                            frame[1] = next_sample;
                        }
                    }

                    // We are done reading one "game frame" of sound
                    if frames_read == BUFFER_LENGTH {
                        match buffer_consumer.pop() {
                            Err(_) => println!("no next frame prepared"),
                            Ok(next_buffer) => consumer = next_buffer,
                        }
                        frames_read = 0;
                    }
                })
            },
            move |err| {
                // react to errors here.
                println!("{}", err);
            },
        )
        .unwrap();

    let mut loop_helper = LoopHelper::builder().build_with_target_rate(FPS as f32);

    let mut frame_count = 0;
    const ROLLBACK_FRAMES: usize = 60;

    std::thread::spawn(move || {
        let mut prev = osci.clone();
        loop {
            if !buffer_producer.is_full() {
                // Allocate a new buffer for the next frame
                let (mut new_producer, new_consumer) = ring_buf();
                buffer_producer.push(new_consumer).unwrap();

                // Write 1 game frame worth of audio into the buffer
                let mut chunk = new_producer.write_chunk_uninit(BUFFER_LENGTH).unwrap();
                let (out, _) = chunk.as_mut_slices();
                out.iter_mut().for_each(|item| {
                    item.write(osci.tick());
                });
                unsafe { chunk.commit_all() };

                frame_count += 1;

                if frame_count > ROLLBACK_FRAMES {
                    println!("rollback");
                    osci = prev.clone();
                    frame_count = 0;
                } else {
                    prev = osci.clone()
                }
            } else {
                // Sound thread hasn't started processing yet, so just sleep
                loop_helper.loop_sleep()
            }
        }
    });

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}
