use gamercade_core::SpriteSheet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorSpriteSheet {
    pub name: String,
    pub sprite_sheet: SpriteSheet,
}
