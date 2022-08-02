use crate::{raw, i32_u32_to_option};

pub fn height() -> usize {
    unsafe { raw::height() as usize}
}
pub fn width() -> usize {
    unsafe { raw::width() as usize}
}
pub fn fps() -> usize {
    unsafe { raw::fps() as usize}
}
pub fn frame_time() -> f32 {
    unsafe { raw::frame_time()}
}
pub fn sprite_sheet_count() -> u8 {
    unsafe { raw::sprite_sheet_count() as u8}
}
pub fn palette_count() -> u8 {
    unsafe { raw::palette_count() as u8}
}
pub fn sprite_height(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_height(sprite_sheet as i32)};
    i32_u32_to_option(val)
}
pub fn sprite_width(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_width(sprite_sheet as i32)};
    i32_u32_to_option(val)
}
pub fn sprite_count(sprite_sheet: usize) -> Option<u32> {
    let val = unsafe { raw::sprite_count(sprite_sheet as i32)};
    i32_u32_to_option(val)
}
