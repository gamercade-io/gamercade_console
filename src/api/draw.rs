use gamercade_core::GraphicsParameters;

use crate::raw;

pub fn clear_screen(graphics_parameters: GraphicsParameters) {
    let gp = graphics_parameters.into();
    unsafe { raw::clear_screen(gp) }
}

pub fn set_pixel(graphics_parameters: GraphicsParameters, x: i32, y: i32) {
    let gp = graphics_parameters.into();
    unsafe { raw::set_pixel(gp, x, y) }
}

pub fn circle(graphics_parameters: GraphicsParameters, x: i32, y: i32, radius: u32) {
    let gp = graphics_parameters.into();
    unsafe { raw::circle(gp, x, y, radius as i32) }
}

pub fn rect(graphics_parameters: GraphicsParameters, x: i32, y: i32, width: u32, height: u32) {
    let gp = graphics_parameters.into();
    unsafe { raw::rect(gp, x, y, width as i32, height as i32) }
}

pub fn rect_filled(
    graphics_parameters: GraphicsParameters,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) {
    let gp = graphics_parameters.into();
    unsafe { raw::rect_filled(gp, x, y, width as i32, height as i32) }
}

pub fn line(graphics_parameters: GraphicsParameters, x0: i32, y0: i32, x1: i32, y1: i32) {
    let gp = graphics_parameters.into();
    unsafe { raw::line(gp, x0, y0, x1, y1) }
}

pub fn sprite(graphics_parameters: GraphicsParameters, transparency_mask: u64, x: i32, y: i32) {
    let gp = graphics_parameters.into();
    unsafe { raw::sprite(gp, transparency_mask as i64, x, y) }
}
