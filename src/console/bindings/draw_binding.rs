use crate::api::{DrawApi, DrawApiBinding};
use crate::console::Contexts;
use paste::paste;
use wasmtime::{Caller, Linker};

macro_rules! derive_draw_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl DrawApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |mut caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data_mut().draw_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_draw_api_binding! {
    clear_screen(graphics_parameters: i32),
    set_pixel(graphics_parameters: i32, x: i32, y: i32),

    circle(graphics_parameters: i32, x: i32, y: i32, radius: i32),
    // fn circle_filled(x: i32, y: i32, color_index: i32, palette_index: i32),

    rect(
        graphics_parameters: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ),

    rect_filled(
        graphics_parameters: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ),

    line(graphics_parameters: i32, x0: i32, y0: i32, x1: i32, y1: i32),

    sprite(
        graphics_parameters: i32,
        transparency_mask: i64,
        x: i32,
        y: i32
    ),
}
