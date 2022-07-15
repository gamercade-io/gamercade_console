use crate::api::GraphicsParameterApiBinding;
use crate::console::Contexts;
use paste::paste;
use wasmtime::{Caller, Linker};

macro_rules! derive_graphics_parameter_bindings {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl GraphicsParameterApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data().graphics_parameter_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_graphics_parameter_bindings! {
    palette_index(palette_index: i32),
    sprite_sheet_index(sprite_sheet_index: i32),
    sprite_index(sprite_index: i32),
    color_index(color_index: i32),
    flip_x(flip_x: i32),
    flip_y(flip_y: i32),
}
