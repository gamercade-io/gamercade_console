use gamercade_core::Palette;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorPalette {
    name: String,
    palette: Palette,
}
