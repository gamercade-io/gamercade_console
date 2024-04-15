use super::{f32_to_option, i32_u32_to_option};
use crate::raw;

/// Returns the height of the screen, in pixels.
pub fn height() -> usize {
    unsafe { raw::height() as usize }
}

/// Returns the width of the screen, in pixels.
pub fn width() -> usize {
    unsafe { raw::width() as usize }
}

/// Returns the target frame rate, in frames per second.
pub fn fps() -> usize {
    unsafe { raw::fps() as usize }
}

/// Returns the time per frame. This is equal to 1.0 / fps.
/// Also known as tick time or delta time.
pub fn frame_time() -> f32 {
    unsafe { raw::frame_time() }
}

/// Returns the total number of sprite sheets present in the ROM.
pub fn sprite_sheet_count() -> u8 {
    unsafe { raw::sprite_sheet_count() as u8 }
}

/// Returns the number of palettes sheets present in the ROM.
pub fn palette_count() -> u8 {
    unsafe { raw::palette_count() as u8 }
}

/// Returns the height of each image for the requested sprite sheet.
/// If the index is invalid, will return None.
pub fn sprite_height(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_height(sprite_sheet as i32) };
    i32_u32_to_option(val)
}

/// Returns the width of each image for the requested sprite sheet.
/// If the index is invalid, will return None.
pub fn sprite_width(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_width(sprite_sheet as i32) };
    i32_u32_to_option(val)
}

/// Returns the number of sprites for the requsted sprite sheet.
/// If the index is invalid, will return None.
pub fn sprite_count(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_count(sprite_sheet as i32) };
    i32_u32_to_option(val)
}

/// Returns the length of the requested song in seconds.
/// If the requested song is invalid, will return None.
pub fn bgm_length_secs(bgm_index: usize) -> Option<f32> {
    let val = unsafe { raw::bgm_length_secs(bgm_index as i32) };
    f32_to_option(val)
}

/// Returns the length of the requested song in frames.
/// Any fractional component will be rounded up.
/// If the requested song is invalid, will return None.
pub fn bgm_length_frames(bgm_index: usize) -> Option<u32> {
    let val = unsafe { raw::bgm_length_frames(bgm_index as i32) };
    i32_u32_to_option(val)
}

/// Returns the length of the requested sfx in seconds.
/// If the requested sfx is invalid, will return None.
pub fn sfx_length_secs(sfx_index: usize) -> Option<f32> {
    let val = unsafe { raw::sfx_length_secs(sfx_index as i32) };
    f32_to_option(val)
}

/// Returns the length of the requested sfx in frames.
/// Any fractional component will be rounded up.
/// If the requested sfx is invalid, will return None.
pub fn sfx_length_frames(sfx_index: usize) -> Option<u32> {
    let val = unsafe { raw::sfx_length_frames(sfx_index as i32) };
    i32_u32_to_option(val)
}
