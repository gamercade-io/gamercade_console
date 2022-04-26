use gamercade_core::{FrameRate, Resolution, Rom};
use serde::{Deserialize, Serialize};

use super::{EditorPalette, EditorSpriteSheet};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub graphics: EditorGraphicsData,
    //pub sounds: SoundsData,
}

impl Default for EditorRom {
    fn default() -> Self {
        todo!()
    }
}

impl EditorRom {
    pub fn export_as_rom(&self) -> Rom {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGraphicsData {
    sprite_sheets: Box<[EditorSpriteSheet]>,
    palettes: Box<[EditorPalette]>,
}
