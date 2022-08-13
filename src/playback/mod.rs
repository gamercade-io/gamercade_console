mod chain_playback;
mod instrument_instance;
mod phrase_playback;
mod song_playback;

pub use chain_playback::*;
pub use instrument_instance::*;
pub use phrase_playback::*;
pub use song_playback::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackerFlow {
    Continue,
    Finished,
}
