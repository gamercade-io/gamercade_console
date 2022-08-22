use crate::raw;

/// Logs text to the console window.
pub fn console_log(text: &str) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::console_log(text.0, text.1) }
}

/// Draw text to the screen
/// This function is UNIMPLEMENTED
pub fn draw_text(text: &str, x: i32, y: i32) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::draw_text(text.0, text.1, x, y) }
}

fn make_wasm_text_ptr(text: &str) -> (i32, i32) {
    (text.as_ptr() as i32, text.len() as i32)
}
