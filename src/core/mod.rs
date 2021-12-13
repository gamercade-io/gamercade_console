mod color;
mod input_state;
mod palette;
mod resolution;
mod rom;
mod sprite;

pub use color::*;
pub use input_state::*;
pub use palette::*;
pub use resolution::*;
pub use rom::*;
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
