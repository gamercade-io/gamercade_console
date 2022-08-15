use crate::api::{AudioApi, AudioApiBinding};
use paste::paste;
use wasmtime::{Caller, Linker};

use crate::console::Contexts;

macro_rules! derive_audio_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl AudioApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data().audio_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_audio_api_binding! {
    play_bgm(bgm_index: i32),
    play_sfx(sfx_index: i32, channel: i32),
    stop_bgm(),
    stop_sfx(channel: i32),
    bgm_is_playing(),
    sfx_is_playing(channel: i32),
}
