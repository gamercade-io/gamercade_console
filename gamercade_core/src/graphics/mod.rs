mod color;
mod frame_rate;
mod graphics_data;
mod graphics_parameters;
mod palette;
mod resolution;
mod sprite_iter;
mod sprites;

pub use color::*;
pub use frame_rate::*;
pub use graphics_data::*;
pub use graphics_parameters::*;
pub use palette::*;
pub use resolution::*;
pub use sprite_iter::*;
pub use sprites::*;

pub const PALETTE_COLORS: usize = 64;
pub const PALETTE_MAX_COUNT: usize = 256;
pub const SPRITE_SHEET_MAX_COUNT: usize = 256;
pub const SPRITE_SHEET_MAX_SPRITES: usize = 256;
