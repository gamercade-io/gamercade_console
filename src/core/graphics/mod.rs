mod color;
mod frame_rate;
mod indices;
mod palette;
mod resolution;
mod sprite;

pub use color::*;
pub use frame_rate::*;
pub use indices::*;
pub use palette::*;
pub use resolution::*;
pub use sprite::*;

pub const PALETTE_COLORS: usize = 16;
pub const PALETTE_COUNT: usize = 256;
