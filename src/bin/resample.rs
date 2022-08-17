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
    source::SamplesConverter,
    DeviceTrait, Source,
};
use rtrb::{Consumer, Producer, RingBuffer};
use rubato::{FftFixedInOut, FftFixedOut, Resampler, ScalarInterpolator, WindowFunction};
use spin_sleep::LoopHelper;

const SAMPLE_RATE: u32 = 44_100; // hard coded to my system
const FPS: u32 = 60;
const BUFFER_LENGTH: usize = (SAMPLE_RATE / FPS) as usize;

// enough to store 1 full "game frame" of audio
fn ring_buf<T>(len: usize) -> (Producer<T>, Consumer<T>) {
    RingBuffer::new(len)
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
    let output_rate = supported_config.sample_rate();
    println!("sample rate: {:?}", output_rate);
    let output_buffer_len = output_rate.0 / FPS;
    let output_buffer_len = output_buffer_len as usize;
    let config = StreamConfig::from(supported_config);

    // Produces buffers full of "frames"
    let (mut buffer_producer, mut buffer_consumer) = RingBuffer::new(2);
    let (mut producer, mut consumer) = ring_buf(output_buffer_len);

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

    let mut resampler = FftFixedInOut::<f32>::new(
        SAMPLE_RATE as usize,
        output_rate.0 as usize,
        output_buffer_len,
        1,
    )
    .unwrap();
    //let mut resampler = ScalarInterpolator::<f32>::new(256, 256, 0.95, WindowFunction::Hann);
    //let sinc = dasp::interpolate::sinc::Sinc::new(ring_buffer::Fixed::from([0.0f32; 100]));
    // let linear = Linear::new(0.0, 0.0);
    // let mut signal = Signal::from_hz_to_hz(osci, linear, SAMPLE_RATE as f64, output_rate.0 as f64);

    let mut output_buffer = vec![Vec::with_capacity(SAMPLE_RATE as usize)];

    std::thread::spawn(move || {
        loop {
            if !buffer_producer.is_full() {
                // Allocate a new buffer for the next frame
                let (mut new_producer, new_consumer) = ring_buf(output_buffer_len);
                buffer_producer.push(new_consumer).unwrap();

                let first_run = vec![(0..BUFFER_LENGTH).map(|_| osci.tick()).collect::<Vec<_>>()];

                resampler
                    .process_into_buffer(&first_run, &mut output_buffer, None)
                    .unwrap();

                // Write 1 game frame worth of audio into the buffer
                let chunk = new_producer.write_chunk_uninit(output_buffer_len).unwrap();
                let iter = output_buffer[0].iter().copied();
                chunk.fill_from_iter(iter);
                // let (out, _) = chunk.as_mut_slices();
                // out.iter_mut().for_each(|item| {
                //     item.write(Signal::next(&mut signal));
                // });
                // unsafe { chunk.commit_all() };
            } else {
                // Sound thread hasn't started processing yet, so just sleep
                loop_helper.loop_sleep()
            }
        }
    });

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}
