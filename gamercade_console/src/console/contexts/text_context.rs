use crate::api::TextApi;

#[derive(Clone, Default)]
pub struct TextContext;

impl TextApi for TextContext {
    fn console_log(&self, text: &str) {
        println!("{}", text);
    }
}
