use gamercade_core::{FrameRate, Rom, Screen};
use serde::{Deserialize, Serialize};

use super::{EditorGraphicsData, EditorSoundData};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorRom {
    pub screen: Screen,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundData,
}

impl Default for EditorRom {
    fn default() -> Self {
        Self {
            player_count: (1, 1),
            screen: Screen::default(),
            frame_rate: FrameRate::default(),
            graphics: EditorGraphicsData::default(),
            sounds: EditorSoundData::default(),
        }
    }
}

impl EditorRom {
    pub fn export_as_rom(&self, code: &[u8]) -> Rom {
        Rom {
            screen: self.screen,
            frame_rate: self.frame_rate,
            graphics: (&self.graphics).into(),
            sounds: (&self.sounds).into(),
            code: code.into(),
            player_count: self.player_count,
        }
    }
}
