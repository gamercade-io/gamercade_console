use gamercade_core::SpriteSheet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorSpriteSheet {
    name: String,
    sprite_sheet: SpriteSheet,
}
