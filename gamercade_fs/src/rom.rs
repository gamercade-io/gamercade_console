use serde::{Deserialize, Serialize};

use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};

use crate::{GameAssetProvider, GameCodeProvider};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: GraphicsData,
    pub sounds: SoundRom,
    pub code: Box<[u8]>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            frame_rate: Default::default(),
            graphics: Default::default(),
            sounds: Default::default(),
            code: Default::default(),
            player_count: (1, 1),
        }
    }
}

impl Rom {
    pub const fn height(&self) -> i32 {
        self.resolution.height()
    }

    pub const fn width(&self) -> i32 {
        self.resolution.width()
    }
}

impl GameCodeProvider for Rom {
    fn code(&self) -> &[u8] {
        &self.code
    }
}

impl GameAssetProvider for Rom {
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
        self.graphics.clone()
    }

    fn sounds(&self) -> SoundRom {
        self.sounds.clone()
    }
}
