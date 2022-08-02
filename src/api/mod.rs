pub mod data;
pub mod draw;
pub mod input;
pub mod multiplayer;
pub mod random;
pub mod text;

mod prelude {
    pub use super::data::*;
    pub use super::draw::*;
    pub use super::input::*;
    pub use super::multiplayer::*;
    pub use super::random::*;
    pub use super::text::*;
    pub use gamercade_core::GraphicsParameters;
}

pub(crate) fn i32_bool_to_option(val: i32) -> Option<bool> {
    match val {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

pub(crate) fn f32_to_option(val: f32) -> Option<f32> {
    if val.is_finite() {
        Some(val)
    } else {
        None
    }
}

pub(crate) fn i32_u32_to_option(val: i32) -> Option<u32> {
    if val < 0 {
        None
    } else {
        Some(val as u32)
    }
}
