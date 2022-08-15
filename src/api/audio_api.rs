pub trait AudioApi {
    fn play_bgm(&self, bgm_index: i32);
    fn play_sfx(&self, sfx_index: i32, channel: i32);
    fn stop_bgm(&self);
    fn stop_sfx(&self, channel: i32);

    fn bgm_is_playing(&self) -> i32;
    fn sfx_is_playing(&self, channel: i32) -> i32;
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
derive_bind_audio_api! {
    bind_play_bgm,
    bind_play_sfx,
    bind_stop_bgm,
    bind_stop_sfx,
    bind_bgm_is_playing,
    bind_sfx_is_playing,
}
