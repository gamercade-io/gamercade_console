use std::{process, sync::Arc, time::Duration};

use arrayvec::ArrayVec;
use gamercade_audio::{
    initialize_globals, Chain, ChainId, EnvelopeDefinition, InstrumentDefinition, InstrumentId,
    PatchDefinition, Phrase, PhraseId, Sfx, Song, SongId, SoundEngine, SoundRom, SoundRomInstance,
    WavetableDefinition, WavetableGenerator, WavetableWaveform,
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

const FPS: usize = 60;

// enough to store 1 full "game frame" of audio
fn ring_buf<T>(len: usize) -> (Producer<T>, Consumer<T>) {
    RingBuffer::new(len)
}

pub fn main() {
    initialize_globals();

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

    let mut engine = test_engine(output_sample_rate);
    engine.play_bgm(Some(SongId(0)));
    // engine.play_sfx(
    //     Some(Sfx {
    //         bpm: 120.0,
    //         chain: ChainId(0),
    //     }),
    //     0,
    // );

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
    let mut render_results = vec![0.0; output_buffer_len];

    std::thread::spawn(move || {
        loop {
            if !buffer_producer.is_full() {
                // Allocate a new buffer for the next frame
                let (mut new_producer, new_consumer) = ring_buf(output_buffer_len);
                buffer_producer.push(new_consumer).unwrap();

                // Generate 1 game frame worth of audio
                engine.render(&mut render_results[0..output_buffer_len]);

                // Write 1 game frame worth of audio into the buffer
                let chunk = new_producer.write_chunk_uninit(output_buffer_len).unwrap();
                chunk.fill_from_iter(render_results.iter().cloned());
            } else {
                // Sound thread hasn't started processing yet, so just sleep
                loop_helper.loop_sleep()
            }
        }
    });

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}

// Initialize our sound sources
// This isn't the intended use case, only a temporary solution until
// the editor gets audio support.
fn test_engine(output_sample_rate: usize) -> SoundEngine {
    let instruments = vec![
        InstrumentDefinition::FMSynth(PatchDefinition::default()),
        InstrumentDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
        }),
    ];

    let mut chains0 = ArrayVec::new();
    chains0.push(Some(PhraseId(0)));
    chains0.push(Some(PhraseId(1)));

    let mut chains1 = ArrayVec::new();
    chains1.push(Some(PhraseId(0)));
    chains1.push(Some(PhraseId(0)));
    chains1.push(Some(PhraseId(2)));
    chains1.push(Some(PhraseId(2)));

    let songs = vec![Song {
        bpm: 140.0,
        tracks: vec![[
            Some(ChainId(0)),
            Some(ChainId(1)),
            None,
            None,
            None,
            None,
            None,
            None,
        ]]
        .into_boxed_slice(),
    }]
    .into_boxed_slice();

    let rom = SoundRom {
        songs,
        chains: vec![Chain { entries: chains0 }, Chain { entries: chains1 }].into_boxed_slice(),
        phrases: vec![
            Phrase::c_scale(InstrumentId(0)),
            Phrase::c_scale_reverse(InstrumentId(0)),
            Phrase::c_scale(InstrumentId(1)),
            Phrase::c_scale_reverse(InstrumentId(1)),
        ]
        .into_boxed_slice(),
        instruments: instruments.into_boxed_slice(),
        sfx: vec![].into_boxed_slice(),
    };

    let rom = Arc::new(SoundRomInstance::new(&rom));

    println!("sound rom instance created");
    SoundEngine::new(&rom, output_sample_rate)
}
