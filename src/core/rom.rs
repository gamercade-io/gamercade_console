use crate::core::graphics::Resolution;
use serde::{Deserialize, Serialize};

use super::{
    graphics::{FrameRate, GraphicsData},
    SoundsData,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub(crate) resolution: Resolution,
    pub(crate) frame_rate: FrameRate,
    pub(crate) graphics: GraphicsData,
    pub(crate) sounds: SoundsData,
    //pub(crate) code: Box<[u8]>,
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
