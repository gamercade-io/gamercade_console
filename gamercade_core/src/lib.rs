mod graphics;
mod input;

pub use graphics::*;
pub use input::*;

pub const BYTES_PER_PIXEL: usize = 4;

pub const MAX_ROM_SIZE: usize = 1_000_000 * 16; // 16mb
