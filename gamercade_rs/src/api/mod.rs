/// Functions related to getting sound engine state, or playing and stopping music or sfx.
pub mod audio;

/// Functions and queries for gathering data about the game ROM itself.
pub mod data;

/// Functions for drawing graphics and primitives to the screen.
pub mod draw;

/// Functions for manipulating graphics parameters efficiently.
pub mod graphics_parameters;

/// Functions for query the state of controllers.
///
/// Input falls into two different categories: Buttons and Axis values.
///
/// Button inputs each have three possible functions, `pressed`, `held`, and `released`
///
/// `pressed` is for checking if a button was just pressed this frame. It will return true
/// on the first frame that the button is pressed, and will return false until the button is released
/// and pressed again.
///
/// `held` is for checking if a button is pressed this frame. It will return true on the first
/// frame it is pressed, and will continue to return true until the button is released.
///
/// `released` is for checking if a button was just released this frame. It will return true
/// on the first frame that the button is release, and will return false until the button is pressed
/// and released again.
///
/// Axis inputs only have a single output: a f32 value associated with their current value.
pub mod input;

/// Functions to query the state of the network session.
pub mod multiplayer;

/// Functions to handle random number generation.
pub mod random;

/// Functions for dealing with text and strings.
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
    if val.is_negative() {
        None
    } else {
        Some(val as u32)
    }
}
