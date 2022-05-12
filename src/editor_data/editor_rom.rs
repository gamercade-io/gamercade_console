use gamercade_core::{FrameRate, Resolution, Rom};
use serde::{Deserialize, Serialize};

use crate::ui::{GraphicsEditor, SoundsEditor};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EditorRom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    // TODO: Something about code?
}

impl EditorRom {
    pub fn export_as_rom(
        &self,
        _graphics_editor: &GraphicsEditor,
        _sounds_editor: &SoundsEditor,
    ) -> Rom {
        todo!("TODO: Export as ROM is not yet implemented")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorGraphicsData {}
