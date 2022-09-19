use serde::{Deserialize, Serialize};

use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};

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
