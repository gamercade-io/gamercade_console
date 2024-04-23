use crate::api::{DrawApi, DrawApiBinding};
use crate::console::Contexts;
use gamercade_core::BYTES_PER_PIXEL;
use paste::paste;
use wasmtime::{Caller, Extern, Linker};

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
                                caller.data_mut().draw_context.$ident($($name,)*)
                        }).unwrap();
                    }
                )*

                fn bind_write_pixel_buffer(&mut self) {
                    self.func_wrap(
                        "env",
                        "write_pixel_buffer",
                        |mut caller: Caller<'_, Contexts>, start_index: i32, parameters_ptr: i32, len: i32| {
                            let mem = match caller.get_export("memory") {
                                Some(Extern::Memory(mem)) => mem,
                                _ => return Err(wasmtime::Error::msg("Failed to find hose memory")),
                            };

                            let (data, store) = mem.data_and_store_mut(&mut caller);

                            let data = match data
                                .get(parameters_ptr as u32 as usize..)
                                .and_then(|arr| arr.get(..len as u32 as usize * BYTES_PER_PIXEL))
                            {
                                Some(data) => bytemuck::cast_slice(data),
                                None => return Err(wasmtime::Error::msg("Invalid data"),)
                            };

                            Ok(store.draw_context.write_pixel_buffer(start_index as usize, data))
                    }).unwrap();
                }
            }
        }
    };
}

derive_draw_api_binding! {
    clear_screen(graphics_parameters: i32),
    set_pixel(graphics_parameters: i32, x: i32, y: i32),

    circle(graphics_parameters: i32, x: i32, y: i32, radius: i32),
    circle_filled(graphics_parameters: i32, x: i32, y: i32, radius: i32),

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
