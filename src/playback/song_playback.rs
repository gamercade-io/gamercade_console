use crate::{ChainPlayback, SongId, SoundEngine, SONG_TRACK_CHANNELS};

#[derive(Clone, Debug)]
pub struct SongPlayback {
    song: SongId,
    chains: [Option<ChainPlayback>; SONG_TRACK_CHANNELS],
}

impl SongPlayback {
    pub fn new() -> Self {
        todo!()
    }

    // TODO: Figure out how to add multiple sound instances
    pub fn update_tracker(&mut self, engine: &SoundEngine) {}
}
