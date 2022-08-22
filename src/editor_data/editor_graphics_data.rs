use serde::{Deserialize, Serialize};

use super::{EditorPalette, EditorSpriteSheet};
use gamercade_core::{GraphicsData, Palette, SpriteSheet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorGraphicsData {
    pub palettes: Vec<EditorPalette>,
    pub sprite_sheets: Vec<EditorSpriteSheet>,
}

impl From<&EditorGraphicsData> for GraphicsData {
    fn from(data: &EditorGraphicsData) -> Self {
        Self {
            sprite_sheets: data
                .sprite_sheets
                .iter()
                .map(|sheet| sheet.sprite_sheet.clone())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            palettes: data
                .palettes
                .iter()
                .map(|palette| palette.palette.clone())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl Default for EditorGraphicsData {
    fn default() -> Self {
        Self {
            palettes: Palette::default_palette_collection()
                .into_iter()
                .map(|(palette, name)| EditorPalette {
                    name: name.to_string(),
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
