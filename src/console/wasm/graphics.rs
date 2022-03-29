use crate::api::{GraphicsApi, GraphicsApiBinding};
use paste::paste;
use wasmtime::{Caller, Linker};

use super::Contexts;

macro_rules! derive_graphics_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl GraphicsApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |mut caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data_mut().graphics_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_graphics_api_binding! {
    clear_screen(color_index: i32, palette_index: i32),
    set_pixel(x: i32, y: i32, color_index: i32, palette_index: i32),
    height(),
    width(),
    circle(x: i32, y: i32, radius: i32, color_index: i32, palette_index: i32),
    //circle_filled(x: i32, y: i32, color_index: i32, palette_index: i32),
    rect(x: i32, y: i32, width: i32, height: i32, color_index: i32, palette_index: i32),
    rect_filled(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    ),
    line(x0: i32, y0: i32, x1: i32, y1: i32, color_index: i32, palette_index: i32),
    //sprite(sprite_index: i32, x: i32, y: i32, palette_index: i32),
}
