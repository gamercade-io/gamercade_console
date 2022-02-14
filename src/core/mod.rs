mod graphics;
mod input;
mod rom;

pub use input::*;
pub use rom::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PaletteIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub usize);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SoundIndex(pub usize);
