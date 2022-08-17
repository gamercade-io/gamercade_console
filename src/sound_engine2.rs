// use std::sync::Arc;

// use rodio::Source;

// use crate::{
//     InstrumentInstanceType, Sfx, SongId, SoundRom, WavetableOscilator, SFX_CHANNELS,
//     SONG_TRACK_CHANNELS, SoundRomInstance,
// };

// #[derive(Clone)]
// pub struct SoundEngine2 {
//     bgm_instruments: [InstrumentInstanceType; SONG_TRACK_CHANNELS],
//     sfx_instruments: [InstrumentInstanceType; SFX_CHANNELS],
//     game_fps: usize,
//     output_sample_rate: u32,
//     rom: Arc<SoundRomInstance>,
// }

// impl SoundEngine2 {
//     pub fn new(rom: &Arc<SoundRomInstance>, game_fps: usize, output_sample_rate: u32) -> Self {
//         use std::array::from_fn;
//         Self {
//             rom: rom.clone(),
//             game_fps,
//             output_sample_rate,
//             bgm_instruments: from_fn(|_| {
//                 InstrumentInstanceType::Wavetable(WavetableOscilator::no_sound())
//             }),
//             sfx_instruments: from_fn(|_| {
//                 InstrumentInstanceType::Wavetable(WavetableOscilator::no_sound())
//             }),
//         }
//     }

//     // Renders the SoundEngine and generates one frame worth samples to be
//     // sent to the audio thread during rendering
//     pub fn render(&mut self, output: &mut Vec<f32>) {
//         let bgm_output = self.bgm_instruments.iter_mut().for_each(|instrument| {
//             let linear = Linear::new(0.0, 0.0);
//             let sample_rate =  instrument.sample_rate() as f64;
//             let mut signal = Signal::from_hz_to_hz(instrument, linear, sample_rate, self.output_sample_rate as f64);

//         });
//     }

//     /// Sets the Bgm to be played. If None is passed in, bgm will be stopped.
//     pub fn play_bgm(&mut self, song: Option<SongId>) {
//         todo!()
//     }

//     /// Sets the Sfx to be played. If None is passed in, the sfx will be stopped.
//     pub fn play_sfx(&mut self, sfx: Option<Sfx>, channel: usize) {
//         todo!()
//     }
// }
