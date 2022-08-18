use std::process;

use arrayvec::ArrayVec;

use gamercade_audio::{
    Chain, ChainId, EnvelopeDefinition, InstrumentDefinition, InstrumentId, PatchDefinition,
    Phrase, PhraseId, Song, SongId, SoundEngine, SoundRom, WavetableDefinition, WavetableGenerator,
    WavetableWaveform,
};

fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        process::exit(1);
    }));

    let mut engine = test_engine();

    engine.play_bgm(Some(SongId(0)));
    //engine.play_sfx(Some(ChainId(0)), 0);

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
            sample_rate: 44_100, //44_100, //44.1 khz
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
        sfx: vec![].into_boxed_slice(),
    };

    SoundEngine::new(&rom)
}
