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
                            |mut caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                let mut context = &mut caller.data_mut().audio_context;
                                context.changed = true;
                                context.$ident($($name as $args,)*)
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
    stop_channel(channel: i32),

    play_note(note_id: i32, instrument_index: i32, channel: i32),
    play_frequency(frequency: f32, instrument_index: i32, channel: i32),
}
