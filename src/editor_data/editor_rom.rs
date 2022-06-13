use gamercade_core::{FrameRate, Resolution, Rom};
use serde::{Deserialize, Serialize};

use super::{EditorGraphicsData, EditorSoundsData};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub graphics: EditorGraphicsData,
    pub sounds: EditorSoundsData,
    // TODO: Something about code?
}

impl EditorRom {
    pub fn export_as_rom(&self) -> Rom {
        todo!("TODO: Export as ROM is not yet implemented")
    }
}
