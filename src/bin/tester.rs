use std::sync::Arc;

use arrayvec::ArrayVec;
use rodio::{OutputStream, Source};

use gamercade_audio::{
    initialize_luts, Chain, ChainId, EnvelopeDefinition, InstrumentDefinition, InstrumentId,
    InstrumentInstance, PatchDefinition, Phrase, PhraseId, Song, SongId, SongPlayback, SoundEngine,
    SoundRom, TrackerFlow, WavetableDefinition, WavetableGenerator, WavetableWaveform,
    PHRASE_STEPS_PER_BEAT,
};

fn main() {
    // Initialization
    initialize_luts();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let engine = test_engine();
    let engine = Arc::new(engine);
    let entries_per_second = 60.0 / engine[SongId(0)].bpm / PHRASE_STEPS_PER_BEAT as f32;
    let entries_per_second = std::time::Duration::from_secs_f32(entries_per_second);

    let playbacks = SongPlayback::generate_multiple(SongId(0), &engine);

    playbacks
        .into_iter()
        .enumerate()
        .for_each(|(index, mut playback)| {
            let engine = engine.clone();
            let instance = InstrumentInstance::from(&engine[InstrumentId(0)]);
            let instance = instance.periodic_access(entries_per_second, move |instance| {
                if TrackerFlow::Finished == playback.update_tracker(&engine, instance) {
                    playback = SongPlayback::new(SongId(0), index, &engine);
                }
            });

            stream_handle.play_raw(instance).unwrap();
        });

    std::thread::sleep(std::time::Duration::from_secs_f32(25.0));
}

// Initialize our sound sources
// This isn't the intended use case, only a temporary solution until
// the editor gets audio support.
fn test_engine() -> SoundEngine {
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
    chains1.push(Some(PhraseId(2)));
    chains1.push(Some(PhraseId(3)));

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
            Phrase::c_scale_reverse(InstrumentId(1)),
            Phrase::c_scale(InstrumentId(1)),
        ]
        .into_boxed_slice(),
        instruments: instruments.into_boxed_slice(),
    };

    SoundEngine::initialize(rom)
}
