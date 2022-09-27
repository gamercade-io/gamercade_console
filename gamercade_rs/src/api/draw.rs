use crate::{prelude::GraphicsParameters, raw};

/// Clears the entire screen, setting the color to the passed in graphics parameter
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn clear_screen(graphics_parameters: GraphicsParameters) {
    unsafe { raw::clear_screen(graphics_parameters.0) }
}

/// Sets the color of a single pixel on the screen, using the passed in graphics parameter.
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn set_pixel(graphics_parameters: GraphicsParameters, x: i32, y: i32) {
    unsafe { raw::set_pixel(graphics_parameters.0, x, y) }
}

/// Draws a circle around point (x, y) on the screen with the passed in radius.
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn circle(graphics_parameters: GraphicsParameters, x: i32, y: i32, radius: u32) {
    unsafe { raw::circle(graphics_parameters.0, x, y, radius as i32) }
}

/// Draws a filled circle around point (x, y) on the screen with the passed in radius.
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn circle_filled(graphics_parameters: GraphicsParameters, x: i32, y: i32, radius: u32) {
    unsafe { raw::circle_filled(graphics_parameters.0, x, y, radius as i32) }
}

/// Draws an empty rectangle with the top left point (x, y) with width and height.
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn rect(graphics_parameters: GraphicsParameters, x: i32, y: i32, width: u32, height: u32) {
    unsafe { raw::rect(graphics_parameters.0, x, y, width as i32, height as i32) }
}

/// Draws a filled rectangle with the top left point (x, y) with width and height.
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn rect_filled(
    graphics_parameters: GraphicsParameters,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) {
    unsafe { raw::rect_filled(graphics_parameters.0, x, y, width as i32, height as i32) }
}

/// Draws a line between point a (x0, y0) to point b (x1, y1).
/// Uses palette_index and color_index. A transparent color will still have it's
/// RGB values used to color the screen.
pub fn line(graphics_parameters: GraphicsParameters, x0: i32, y0: i32, x1: i32, y1: i32) {
    unsafe { raw::line(graphics_parameters.0, x0, y0, x1, y1) }
}

/// Draws a sprite using the passed in graphics parameter, with the top left point (x, y)
/// Uses palette_index, sprite_sheet_index, and sprite_index. Specific color indicies can
/// be enabled or disabled by using the transparency mask.
/// Transparent colors will never be drawn.
pub fn sprite(graphics_parameters: GraphicsParameters, transparency_mask: u64, x: i32, y: i32) {
    unsafe { raw::sprite(graphics_parameters.0, transparency_mask as i64, x, y) }
}

/// Writes multiple pixels into the frame buffer, starting from the passed index and will
/// continue until the data slice has been consumed.
pub fn write_pixel_buffer(start_index: usize, data: &[GraphicsParameters]) {
    unsafe { raw::write_pixel_buffer(start_index as i32, data.as_ptr() as i32, data.len() as i32) }
}
