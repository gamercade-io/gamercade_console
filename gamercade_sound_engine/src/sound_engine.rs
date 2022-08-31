use std::sync::Arc;

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use gamercade_audio::InstrumentId;
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

pub enum SoundEngineChannelType {
    SoundEngineData(Box<SoundEngineData>),
    SoundRomInstance(Arc<SoundRomInstance>),
    PianoKeyPressed {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    PianoKeyReleased {
        channel: usize,
    },
    TriggerNote {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
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
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_note(note);
        }
    }

    pub fn set_key_active(&mut self, active: bool, channel: usize) {
        if let Some(target) = self.sfx.get_mut(channel) {
            target
                .chain_playback
                .phrase_playback
                .instrument
                .set_active(active)
        }
    }

    pub fn trigger_note(&mut self, note: i32, instrument_index: usize, channel: usize) {
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.trigger();
            target.set_note(note);
        }
    }

    pub fn play_frequency(&mut self, frequency: f32, instrument_index: usize, channel: usize) {
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_frequency(frequency);
        }
    }

    pub(crate) fn fast_forward(&mut self, frames: usize) {
        (0..frames).for_each(|_| {
            self.bgm.tick();
            self.sfx.iter_mut().for_each(|sfx| {
                sfx.tick();
            });
        });
    }

    pub fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.rom = new_rom.clone();

        self.bgm.replace_sound_rom_instance(new_rom);
        self.sfx
            .iter_mut()
            .for_each(|sfx| sfx.replace_sound_rom_instance(new_rom));
    }
}

pub struct SoundEngine {
    _stream: Stream,
    sound_frames_per_render_frame: usize,
    producer: Producer<SoundEngineChannelType>,
    output_sample_rate: usize,
}

impl SoundEngine {
    pub fn output_sample_rate(&self) -> usize {
        self.output_sample_rate
    }

    pub fn new(fps: usize, rom: &Arc<SoundRomInstance>, message_buffer_size: usize) -> Self {
        initialize_globals();
        let device = default_host().default_output_device().unwrap();

        let supported_config = device.default_output_config().unwrap();
        let output_sample_rate = supported_config.sample_rate().0 as usize;
        let channels = supported_config.channels() as usize;
        let config = StreamConfig::from(supported_config);

        println!("Output Sample Rate: {}", output_sample_rate);
        println!("Output channels: {}", channels);

        let sound_frames_per_render_frame = output_sample_rate / fps;
        let (producer, mut consumer) = RingBuffer::new(message_buffer_size);
        let mut data = SoundEngineData::new(output_sample_rate, rom);

        let stream = device
            .build_output_stream(
                &config,
                move |frames: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // react to stream events and read or write stream data here.
                    frames.chunks_exact_mut(channels).for_each(|frame| {
                        while let Ok(next_data) = consumer.pop() {
                            match next_data {
                                SoundEngineChannelType::SoundEngineData(next_data) => {
                                    data = *next_data;
                                }
                                SoundEngineChannelType::SoundRomInstance(new_rom) => {
                                    data.replace_sound_rom_instance(&new_rom);
                                }
                                SoundEngineChannelType::PianoKeyPressed {
                                    note_index,
                                    instrument_index,
                                    channel,
                                } => data.play_note(note_index as i32, instrument_index, channel),
                                SoundEngineChannelType::PianoKeyReleased { channel } => {
                                    data.set_key_active(false, channel)
                                }
                                SoundEngineChannelType::TriggerNote {
                                    note_index,
                                    instrument_index,
                                    channel,
                                } => {
                                    data.trigger_note(note_index as i32, instrument_index, channel)
                                }
                            };
                        }

                        let bgm_frame = data.bgm.tick().iter().sum::<f32>();
                        let sfx_frame = data.sfx.iter_mut().map(|sfx| sfx.tick()).sum::<f32>();
                        let output =
                            (bgm_frame + sfx_frame) / (SFX_CHANNELS + SONG_TRACK_CHANNELS) as f32;

                        frame.iter_mut().for_each(|channel| {
                            *channel = output;
                        });
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
        data.fast_forward(self.sound_frames_per_render_frame);
    }

    pub fn sync_audio_thread(&mut self, data: &SoundEngineData) {
        self.producer
            .push(SoundEngineChannelType::SoundEngineData(Box::new(
                data.clone(),
            )))
            .unwrap()
    }

    pub fn send(&mut self, message: SoundEngineChannelType) {
        self.producer.push(message).unwrap();
    }
}
