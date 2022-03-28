use crate::core::graphics::{Palette, Resolution, Sprite};
use serde::{Deserialize, Serialize};

use super::graphics::FrameRate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub(crate) resolution: Resolution,
    pub(crate) frame_rate: FrameRate,
    pub(crate) graphics: GraphicsData,
    pub(crate) sounds: SoundsData,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Resolution::Low,
            frame_rate: FrameRate::Fast,
            graphics: GraphicsData::default(),
            sounds: SoundsData {},
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsData {
    pub(crate) sprites: Box<[Sprite]>,
    pub(crate) palettes: Box<[Palette]>,
}

impl Default for GraphicsData {
    fn default() -> Self {
        Self {
            sprites: vec![].into_boxed_slice(),
            palettes: Palette::default_palette_collection().into_boxed_slice(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundsData {
    //TODO: This
}
