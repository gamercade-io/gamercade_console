use crate::api::TextApi;

#[derive(Clone, Default)]
pub struct TextContext;

impl TextApi for TextContext {
    fn log(&self, text: &str) {
        println!("{}", text);
    }

    fn draw_text(&self, text: &str, x: i32, y: i32) {
        println!("TODO: draw_text at (x{}, y{}): {}", x, y, text);
    }
}
