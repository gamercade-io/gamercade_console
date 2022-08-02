pub mod data;
pub mod draw;
pub mod input;
pub mod multiplayer;
pub mod random;
pub mod text;

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
