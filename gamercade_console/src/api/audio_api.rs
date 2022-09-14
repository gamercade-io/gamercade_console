pub trait AudioApi {
    fn play_bgm(&mut self, bgm_index: i32);
    fn play_sfx(&mut self, sfx_index: i32, channel: i32);

    fn stop_bgm(&mut self);
    fn stop_channel(&mut self, channel: i32);

    fn play_note(&mut self, note_id: i32, instrument_index: i32, channel: i32);
    fn play_frequency(&mut self, frequency: f32, instrument_index: i32, channel: i32);
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

derive_bind_audio_api! {
    bind_play_bgm,
    bind_play_sfx,
    bind_stop_bgm,
    bind_stop_channel,
    bind_play_note,
    bind_play_frequency,
}
