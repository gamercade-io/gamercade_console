pub trait TextApi {
    fn console_log(&self, text: &str);
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
    bind_console_log,
}
