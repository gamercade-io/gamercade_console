use paste::paste;

pub trait TextApi {
    fn console_log(&self, text: &str);
}

macro_rules! derive_bind_text_api {
    ($($name:ident,)*) => {
        paste! {
            pub trait TextApiBinding {
                $(
                    fn $name(&mut self);
                    fn [<$name _utf16>](&mut self);
                )*

                fn bind_text_api(&mut self) {
                    $(
                        self.$name();
                        self.[<$name _utf16>]();
                    )*
                }
            }
        }
    }
}

derive_bind_text_api! {
    bind_console_log,
}
