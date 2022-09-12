use crate::raw;

/// Logs text to the console window.
pub fn console_log(text: &str) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::console_log(text.0, text.1) }
}

fn make_wasm_text_ptr(text: &str) -> (i32, i32) {
    (text.as_ptr() as i32, text.len() as i32)
}
