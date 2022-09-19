use gamercade_core::Palette;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorPalette {
    pub name: String,
    pub palette: Palette,
}
