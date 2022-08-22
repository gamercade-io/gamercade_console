use std::sync::Arc;

use crate::{
    ChainPlayback, InstrumentInstance, Sfx, SfxPlayback, SongId, SongPlayback, SoundRomInstance,
    SFX_CHANNELS, SONG_TRACK_CHANNELS,
};

#[derive(Clone)]
pub struct SoundEngine {
    bgm: SongPlayback,
    sfx: [SfxPlayback; SFX_CHANNELS],
}

impl SoundEngine {
    pub fn new(rom: &Arc<SoundRomInstance>, output_sample_rate: usize) -> Self {
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
        }
    }

    // Renders the SoundEngine and generates one frame worth samples to be
    // sent to the audio thread during rendering
    pub fn render(&mut self, output: &mut [f32]) {
        output.iter_mut().for_each(|output| {
            let bgm_frame = self.bgm.tick().iter().sum::<f32>();
            let sfx_frame = self.sfx.iter_mut().map(|sfx| sfx.tick()).sum::<f32>();

            *output = (bgm_frame + sfx_frame) / (SFX_CHANNELS + SONG_TRACK_CHANNELS) as f32;
        })
    }

    /// Sets the Bgm to be played. If None is passed in, bgm will be stopped.
    pub fn play_bgm(&mut self, song: Option<SongId>) {
        self.bgm.set_song_id(song);
    }

    /// Sets the Sfx to be played. If None is passed in, the sfx will be stopped.
    pub fn play_sfx(&mut self, sfx: Option<Sfx>, channel: usize) {
        self.sfx[channel].set_sfx_id(sfx);
    }
}
