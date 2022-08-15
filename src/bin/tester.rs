use std::{process, sync::Arc};

use arrayvec::ArrayVec;
use rodio::OutputStream;

use gamercade_audio::{
    initialize_luts, Chain, ChainId, ChainPlayback, EnvelopeDefinition, InstrumentDefinition,
    InstrumentId, InstrumentInstance, InstrumentInstanceType, PatchDefinition, Phrase, PhraseId,
    Song, SongId, SongPlayback, SoundRom, SoundRomInstance, TrackerFlow, WavetableDefinition,
    WavetableGenerator, WavetableWaveform, PHRASE_STEPS_PER_BEAT,
};

fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        process::exit(1);
    }));

    // Initialization
    initialize_luts();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let engine = test_engine();
    let engine = Arc::new(engine);
    let entries_per_second = 60.0 / engine[SongId(0)].bpm / PHRASE_STEPS_PER_BEAT as f32;
    let entries_per_second = std::time::Duration::from_secs_f32(entries_per_second);

    // Test with a single chain
    let tracks = std::array::from_fn(|_| {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let instance = InstrumentInstance {
            receiver,
            instance_type: InstrumentInstanceType::from(&engine[InstrumentId(0)]),
        };

        stream_handle.play_raw(instance).unwrap();
        ChainPlayback::new(None, sender, &engine)
    });

    let mut song = SongPlayback::new(Some(SongId(0)), tracks, &engine);

    // Tracker thread
    std::thread::spawn(move || loop {
        std::thread::sleep(entries_per_second);

        // println!("{:?}", BgmState::new(&song));
        if TrackerFlow::Finished == song.update_tracker() {
            song.set_song_id(Some(SongId(0)));
        }

        //println!("{:?}", TrackerState::new(&chain));
        // if TrackerFlow::Finished == chain.update_tracker() {
        //     chain.set_chain_id(Some(ChainId(1)));
        // }
    });

    std::thread::sleep(std::time::Duration::from_secs_f32(25.0));
}

// Initialize our sound sources
// This isn't the intended use case, only a temporary solution until
// the editor gets audio support.
fn test_engine() -> SoundRomInstance {
    let instruments = vec![
        InstrumentDefinition::FMSynth(PatchDefinition::default()),
        InstrumentDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            sample_rate: 44_100, //44.1 khz
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
        bpm: 120.0,
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
    };

    SoundRomInstance::initialize(rom)
}
