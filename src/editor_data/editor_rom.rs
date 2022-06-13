use gamercade_core::{FrameRate, Resolution, Rom};
use serde::{Deserialize, Serialize};

use super::{EditorGraphicsData, EditorSoundsData};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundsData,
    // TODO: Allow loading/setting a file for code?
}

impl EditorRom {
    pub fn export_as_rom(&self, code: &[u8]) -> Rom {
        Rom {
            resolution: self.resolution,
            frame_rate: self.frame_rate,
            graphics: (&self.graphics).into(),
            sounds: (&self.sounds).into(),
            code: code.into(),
        }
    }
}
