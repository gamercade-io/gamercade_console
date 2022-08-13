use crate::{
    ChainPlayback, InstrumentInstance, SongId, SoundEngine, TrackerFlow, SONG_TRACK_CHANNELS,
};

#[derive(Clone, Debug)]
pub struct SongPlayback {
    song: SongId,
    chain_index: usize,   // The current location in the song
    channel_index: usize, // The channel this playback is responsible for
    chain: Option<ChainPlayback>,
}

impl SongPlayback {
    pub fn generate_multiple(
        song: SongId,
        engine: &SoundEngine,
    ) -> [SongPlayback; SONG_TRACK_CHANNELS] {
        std::array::from_fn(|i| Self::new(song, i, engine))
    }

    pub fn new(song: SongId, channel_index: usize, engine: &SoundEngine) -> Self {
        let mut out = Self {
            song,
            channel_index,
            chain_index: 0,
            chain: None,
        };

        out.fetch_chain_playback(engine);

        out
    }

    fn fetch_chain_playback(&mut self, engine: &SoundEngine) {
        if let Some(chains) = engine[self.song].tracks.get(self.chain_index) {
            self.chain = chains[self.channel_index].map(|next| ChainPlayback::new(next, engine))
        }
    }

    pub fn update_tracker(
        &mut self,
        engine: &SoundEngine,
        instance: &mut InstrumentInstance,
    ) -> TrackerFlow {
        match &mut self.chain {
            Some(chain) => chain.update_tracker(engine, instance),
            None => TrackerFlow::Finished,
        }
    }

    pub fn next_step(&mut self, engine: &SoundEngine) -> TrackerFlow {
        self.chain_index += 1;

        let out = if self.chain_index >= engine[self.song].tracks.len() {
            self.chain_index = 0;
            TrackerFlow::Finished
        } else {
            TrackerFlow::Continue
        };

        self.fetch_chain_playback(engine);
        out
    }
}
