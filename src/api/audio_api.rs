pub trait AudioApi {
    //TODO: Write these
}

macro_rules! derive_bind_audio_api {
    ($($name:ident,)*) => {
        pub trait AudioApiBinding {
            $(fn $name(&mut self);)*

            fn bind_audio_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

// TODO: Write these
derive_bind_audio_api! {}
