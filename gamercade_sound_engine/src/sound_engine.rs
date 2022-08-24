use std::sync::Arc;

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use rtrb::{Producer, RingBuffer};

use crate::{
    initialize_globals, ChainPlayback, InstrumentInstance, SfxPlayback, SongPlayback,
    SoundRomInstance,
};
pub use gamercade_audio::{Sfx, SongId, SFX_CHANNELS, SONG_TRACK_CHANNELS};
#[derive(Clone)]
pub struct SoundEngineData {
    pub bgm: SongPlayback,
    pub sfx: [SfxPlayback; SFX_CHANNELS],
    rom: Arc<SoundRomInstance>,
}

impl SoundEngineData {
    pub fn new(output_sample_rate: usize, rom: &Arc<SoundRomInstance>) -> Self {
        use std::array::from_fn;

        let bgm_tracks = from_fn(|_| {
            ChainPlayback::new(None, rom, InstrumentInstance::no_sound(output_sample_rate))
        });

        Self {
            bgm: SongPlayback::new(None, bgm_tracks, rom, output_sample_rate),
            sfx: from_fn(|_| {
                SfxPlayback::new(
                    None,
                    rom,
                    InstrumentInstance::no_sound(output_sample_rate),
                    output_sample_rate,
                )
            }),
            rom: rom.clone(),
        }
    }

    /// Sets the Bgm to be played. If None is passed in, bgm will be stopped.
    pub fn play_bgm(&mut self, song: Option<SongId>) {
        self.bgm.set_song_id(song);
    }

    /// Sets the Sfx to be played. If None is passed in, the sfx will be stopped.
    pub fn play_sfx(&mut self, sfx: Option<Sfx>, channel: usize) {
        self.sfx[channel].set_sfx_id(sfx);
    }

    pub fn play_note(&mut self, note: i32, instrument_index: usize, channel: usize) {
        let instrument = self.rom.instrument_bank.get(instrument_index);
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_note(note);
        }
    }

    pub fn play_frequency(&mut self, frequency: f32, instrument_index: usize, channel: usize) {
        let instrument = self.rom.instrument_bank.get(instrument_index);
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_frequency(frequency);
        }
    }
}

pub struct SoundEngine {
    _stream: Stream,
    sound_frames_per_render_frame: usize,
    producer: Producer<SoundEngineData>,
    output_sample_rate: usize,
}

impl SoundEngine {
    pub fn output_sample_rate(&self) -> usize {
        self.output_sample_rate
    }

    pub fn new(fps: usize, rom: &Arc<SoundRomInstance>, max_rollback_frames: usize) -> Self {
        initialize_globals();
        let device = default_host().default_output_device().unwrap();

        let supported_config = device
            .supported_output_configs()
            .unwrap()
            .next()
            .unwrap()
            .with_max_sample_rate();
        let output_sample_rate = supported_config.sample_rate().0 as usize;
        let config = StreamConfig::from(supported_config);

        let sound_frames_per_render_frame = output_sample_rate / fps;
        let (producer, mut consumer) = RingBuffer::new(max_rollback_frames);
        let mut data = SoundEngineData::new(output_sample_rate, rom);

        let stream = device
            .build_output_stream(
                &config,
                move |frames: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // react to stream events and read or write stream data here.
                    frames.chunks_exact_mut(2).for_each(|frame| {
                        // TODO: Maybe add some logic to shift
                        // the read index forward/backward due to
                        // delays in threading/CPU, to reduce clicking
                        // Will probably need to tick() the sound sources
                        // to catch them up
                        while let Ok(next_data) = consumer.pop() {
                            data = next_data
                        }

                        let bgm_frame = data.bgm.tick().iter().sum::<f32>();
                        let sfx_frame = data.sfx.iter_mut().map(|sfx| sfx.tick()).sum::<f32>();
                        let output =
                            (bgm_frame + sfx_frame) / (SFX_CHANNELS + SONG_TRACK_CHANNELS) as f32;

                        frame[0] = output;
                        frame[1] = output;
                    })
                },
                move |err| {
                    // react to errors here.
                    println!("{}", err);
                },
            )
            .unwrap();

        stream.play().unwrap();

        Self {
            sound_frames_per_render_frame,
            output_sample_rate,
            _stream: stream,
            producer,
        }
    }

    /// Fast-forwards the the SoundEngineData by generating one frame worth samples
    /// This keeps it somewhat in sync with the audio that's actually being played
    pub fn fast_forward(&mut self, data: &mut SoundEngineData) {
        (0..self.sound_frames_per_render_frame).for_each(|_| {
            data.bgm.tick();
            data.sfx.iter_mut().for_each(|sfx| {
                sfx.tick();
            });
        });
    }

    pub fn sync_audio_thread(&mut self, data: &SoundEngineData) {
        self.producer.push(data.clone()).unwrap()
    }
}
