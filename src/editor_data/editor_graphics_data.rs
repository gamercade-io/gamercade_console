use serde::{Deserialize, Serialize};

use super::{EditorPalette, EditorSpriteSheet};
use gamercade_core::{Palette, SpriteSheet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGraphicsData {
    pub palettes: Vec<EditorPalette>,
    pub sprite_sheets: Vec<EditorSpriteSheet>,
}

impl Default for EditorGraphicsData {
    fn default() -> Self {
        Self {
            palettes: Palette::default_palette_collection()
                .into_iter()
                .enumerate()
                .map(|(index, palette)| EditorPalette {
                    name: format!("Palette {}", index + 1),
                    palette,
                })
                .collect(),
            sprite_sheets: vec![EditorSpriteSheet {
                name: "Sprite Sheet 1".to_string(),
                sprite_sheet: SpriteSheet::default(),
            }],
        }
    }
}
