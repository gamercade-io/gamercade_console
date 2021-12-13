mod color;
mod palette;
mod sprite;

pub use color::*;
pub use palette::*;
pub use sprite::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PaletteIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SoundIndex(pub usize);
