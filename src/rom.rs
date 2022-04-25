use super::graphics::Resolution;
use serde::{Deserialize, Serialize};

use super::{
    graphics::{FrameRate, GraphicsData},
    SoundsData,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub graphics: GraphicsData,
    pub sounds: SoundsData,
    //pub code: Box<[u8]>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Resolution::Low,
            frame_rate: FrameRate::Fast,
            graphics: GraphicsData::default(),
            sounds: SoundsData::default(),
        }
    }
}
