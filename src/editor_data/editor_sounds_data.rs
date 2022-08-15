use gamercade_audio::SoundRom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorSoundData {}

impl From<&EditorSoundData> for SoundRom {
    fn from(_data: &EditorSoundData) -> Self {
        Self {
            songs: Default::default(),
            chains: Default::default(),
            phrases: Default::default(),
            instruments: Default::default(),
            sfx: Default::default(),
        }
    }
}
