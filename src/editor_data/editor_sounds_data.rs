use gamercade_core::SoundsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorSoundsData {}

impl From<&EditorSoundsData> for SoundsData {
    fn from(_data: &EditorSoundsData) -> Self {
        Self {}
    }
}
