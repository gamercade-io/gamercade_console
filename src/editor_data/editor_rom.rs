use gamercade_core::{FrameRate, Resolution, Rom};
use serde::{Deserialize, Serialize};

use super::{EditorGraphicsData, EditorSoundsData};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundsData,
}

impl Default for EditorRom {
    fn default() -> Self {
        Self {
            player_count: (1, 1),
            resolution: Resolution::default(),
            frame_rate: FrameRate::default(),
            graphics: EditorGraphicsData::default(),
            sounds: EditorSoundsData::default(),
        }
    }
}

impl EditorRom {
    pub fn export_as_rom(&self, code: &[u8]) -> Rom {
        Rom {
            resolution: self.resolution,
            frame_rate: self.frame_rate,
            graphics: (&self.graphics).into(),
            sounds: (&self.sounds).into(),
            code: code.into(),
            player_count: self.player_count,
        }
    }
}
