mod color;
mod frame_rate;
mod graphics_data;
mod palette;
mod resolution;
mod sprites;

pub use color::*;
pub use frame_rate::*;
pub use graphics_data::*;
pub use palette::*;
pub use resolution::*;
pub use sprites::*;

pub const PALETTE_COLORS: usize = 16;
pub const PALETTE_MAX_COUNT: usize = 256;
