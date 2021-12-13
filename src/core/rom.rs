use crate::core::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub sprites: Box<[Sprite]>,
    pub palettes: Box<[Palette]>,
    pub sounds: Sounds,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Resolution::Low,
            sprites: vec![].into_boxed_slice(),
            palettes: vec![Palette::bubblegum16()].into_boxed_slice(),
            sounds: Sounds {},
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sounds {
    //TODO: This
}
