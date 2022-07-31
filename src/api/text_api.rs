// TODO: Write this
pub trait TextApi {
    fn log(&self, text: &str);
    fn draw_text(&self, text: &str, x: i32, y: i32);
}

macro_rules! derive_bind_text_api {
    ($($name:ident,)*) => {
        pub trait TextApiBinding {
            $(fn $name(&mut self);)*

            fn bind_text_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

derive_bind_text_api! {
    bind_log,
    bind_draw_text,
}
