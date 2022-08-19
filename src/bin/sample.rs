use std::{process, sync::Arc, time::Duration};

use gamercade_audio::{EnvelopeDefinition, SampleBitDepth, SampleDefinition, SamplerInstance};

use hound::WavReader;
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

const FPS: usize = 60;

// enough to store 1 full "game frame" of audio
fn ring_buf<T>(len: usize) -> (Producer<T>, Consumer<T>) {
    RingBuffer::new(len)
}

fn sampler(output_sample_rate: usize) -> SamplerInstance {
    let reader = WavReader::open("CantinaBand3.wav").unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let source_sample_rate = spec.sample_rate as usize;
    let data = reader
        .into_samples::<SampleBitDepth>()
        .flatten()
        .collect::<Vec<_>>();

    println!("bit depth: {:?}", spec.bits_per_sample);
    println!("sample format: {:?}", spec.sample_format);
    println!("channels: {}", channels);
    println!("source sample rate {:?}", source_sample_rate);

    SamplerInstance::new(
        &Arc::new(SampleDefinition {
            data: data.into_boxed_slice(),
            source_sample_rate,
            sample_frequency: None,
            envelope_definition: EnvelopeDefinition::default(),
        }),
        output_sample_rate,
    )
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
    let output_sample_rate = supported_config.sample_rate().0 as usize;
    println!("sample rate: {:?}", output_sample_rate);
    let config = StreamConfig::from(supported_config);

    let output_buffer_len = output_sample_rate / FPS;

    // Produces buffers full of "frames"
    let (mut buffer_producer, mut buffer_consumer) = RingBuffer::new(2);
    let (mut producer, mut consumer) = ring_buf(output_buffer_len);

    // Write silence for testing
    producer
        .write_chunk_uninit(output_buffer_len)
        .unwrap()
        .fill_from_iter(Some(0.0).iter().cycle().cloned());

    let mut sample = sampler(output_sample_rate);
    sample.set_active(true);

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
                    if frames_read == output_buffer_len {
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

    std::thread::spawn(move || {
        loop {
            if !buffer_producer.is_full() {
                // Allocate a new buffer for the next frame
                let (mut new_producer, new_consumer) = ring_buf(output_buffer_len);
                buffer_producer.push(new_consumer).unwrap();

                // Write 1 game frame worth of audio into the buffer
                let mut chunk = new_producer.write_chunk_uninit(output_buffer_len).unwrap();
                let (out, _) = chunk.as_mut_slices();
                out.iter_mut().for_each(|item| {
                    item.write(sample.tick());
                });
                unsafe { chunk.commit_all() };
            } else {
                // Sound thread hasn't started processing yet, so just sleep
                loop_helper.loop_sleep()
            }
        }
    });

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}
