use crate::api::{DataApi, DataApiBinding};
use crate::console::Contexts;
use paste::paste;
use wasmtime::{Caller, Linker};

macro_rules! derive_data_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl DataApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data().data_context.$ident($($name,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_data_api_binding! {
    height(),
    width(),
    fps(),
    frame_time(),
    sprite_sheet_count(),
    palette_count(),
    sprite_height(sprite_sheet: i32),
    sprite_width(sprite_sheet: i32),
    sprite_count(sprite_sheet: i32),
    bgm_length_secs(bgm_index: i32),
    bgm_length_frames(bgm_index: i32),
    sfx_length_secs(sfx_index: i32),
    sfx_length_frames(sfx_index: i32),
}
