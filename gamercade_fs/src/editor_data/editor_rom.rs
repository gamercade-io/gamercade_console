use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};
use serde::{Deserialize, Serialize};

use crate::GameAssetProvider;

use super::{EditorGraphicsData, EditorSoundData};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundData,
}

impl Default for EditorRom {
    fn default() -> Self {
        Self {
            player_count: (1, 1),
            resolution: Resolution::default(),
            frame_rate: FrameRate::default(),
            graphics: EditorGraphicsData::default(),
            sounds: EditorSoundData::default(),
        }
    }
}

impl GameAssetProvider for EditorRom {
    fn resolution(&self) -> Resolution {
        self.resolution
    }

    fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }

    fn player_count(&self) -> (usize, usize) {
        self.player_count
    }

    fn graphics(&self) -> GraphicsData {
        (&self.graphics).into()
    }

    fn sounds(&self) -> SoundRom {
        (&self.sounds).into()
    }
}
