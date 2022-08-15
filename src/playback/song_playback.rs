use std::sync::Arc;

use crate::{
    BgmState, ChainPlayback, SongId, SoundRomInstance, Ticker, TrackerFlow, SONG_TRACK_CHANNELS,
};

#[derive(Debug)]
pub struct SongPlayback {
    pub(crate) song: Option<SongId>,
    pub(crate) chain_index: usize, // The current location in the song
    pub(crate) tracks: [ChainPlayback; SONG_TRACK_CHANNELS],
    pub(crate) chain_states: [TrackerFlow; SONG_TRACK_CHANNELS],
    pub(crate) rom: Arc<SoundRomInstance>,
    pub(crate) ticker: Arc<Ticker>,
}

fn default_chain_states() -> [TrackerFlow; SONG_TRACK_CHANNELS] {
    std::array::from_fn(|_| TrackerFlow::Continue)
}

impl SongPlayback {
    pub(crate) fn new(
        song: Option<SongId>,
        tracks: [ChainPlayback; SONG_TRACK_CHANNELS],
        rom: &Arc<SoundRomInstance>,
        ticker: &Arc<Ticker>,
    ) -> Self {
        let mut out = Self {
            song,
            chain_index: 0,
            tracks,
            rom: rom.clone(),
            chain_states: default_chain_states(),
            ticker: ticker.clone(),
        };

        out.set_song_id(song);
        out
    }

    /// Sets this playback to play specified Song Id.
    /// Passing in None will mute the playback.
    pub(crate) fn set_song_id(&mut self, song: Option<SongId>) {
        self.song = song;
        self.chain_index = 0;

        // If the song is valid, update all chains to
        // use the correct indices and data
        if let Some(song) = song {
            let next_chain = self.rom[song].tracks[0];
            self.chain_states = default_chain_states();
            self.tracks
                .iter_mut()
                .zip(next_chain.iter())
                .for_each(|(track, next)| {
                    track.set_chain_id(*next);
                });
        }
    }

    /// Updates this song to match that of the passed in BgmState
    /// Useful when trying to seek to an exact time.
    pub(crate) fn set_from_song_state(&mut self, bgm_state: &BgmState) {
        self.song = bgm_state.song_id;
        self.chain_index = bgm_state.chain_index;
        self.ticker.write_from_state(&bgm_state.bgm_ticker);

        bgm_state
            .chain_states
            .iter()
            .zip(self.tracks.iter_mut().zip(self.chain_states.iter_mut()))
            .for_each(|(next, (track, state))| {
                *state = TrackerFlow::Continue;
                track.set_from_chain_state(next);
            });
    }

    /// Calls update_tracker on each chain playback,
    /// if all are done, will increment our current chain index
    /// within the song
    pub(crate) fn update_tracker(&mut self) -> TrackerFlow {
        // Call update on each of the chains, but
        // only if they should continue playing
        self.tracks
            .iter_mut()
            .zip(self.chain_states.iter_mut())
            .for_each(|(tracker, state)| {
                if TrackerFlow::Continue == *state {
                    *state = tracker.update_tracker()
                }
            });

        if self
            .chain_states
            .iter()
            .all(|state| *state == TrackerFlow::Finished)
        {
            self.next_step()
        } else {
            TrackerFlow::Continue
        }
    }

    /// Advances the tracks to the next chain within the song.
    pub(crate) fn next_step(&mut self) -> TrackerFlow {
        // Song doesn't exist, so we're done
        if self.song.is_none() {
            return TrackerFlow::Finished;
        };
        let song = self.song.unwrap();

        self.chain_index += 1;

        // Song doesn't have any more entries, so we're done
        let next_chain = self.rom[song].tracks.get(self.chain_index);
        if next_chain.is_none() {
            return TrackerFlow::Finished;
        }

        let next_chain = next_chain.unwrap();

        self.tracks
            .iter_mut()
            .zip(self.chain_states.iter().zip(next_chain.iter()))
            .for_each(|(track, (state, next))| {
                if TrackerFlow::Continue == *state {
                    track.set_chain_id(*next)
                }
            });

        TrackerFlow::Continue
    }
}
