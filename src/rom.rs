use crate::{core::*, RomMap};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    resoltuion: Resolution,
    code: String,
    graphics: Graphics,
    sounds: Sounds,
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub enum Resolution {
    Low,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graphics {
    sprites: RomMap<SpriteIndex, Sprite>,
    palettes: RomMap<PaletteIndex, Palette>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sounds {
    //TODO: This
}