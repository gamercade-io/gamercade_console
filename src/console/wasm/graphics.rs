use crate::{
    api::{GraphicsApi, GraphicsApiBinding},
    console::GraphicsContext,
};
use paste::paste;
use wasmer::Function;

use super::WasmConsoleBuilder;

macro_rules! derive_graphics_api_binding {
    ($($ident:ident,)*) => {
        paste! {
            impl GraphicsApiBinding for WasmConsoleBuilder<'_> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.imports.push((
                            stringify!($ident),
                            Function::new_native_with_env(
                                self.store,
                                self.graphics_context.clone(),
                                GraphicsContext::$ident)
                            ));
                    }
                )*
            }
        }
    }
}

derive_graphics_api_binding! {
    clear_screen,
    set_pixel,
    height,
    width,
    //circle,
    //circle_filled,
    rect,
    //rect_filled,
    line,
    //sprite,
}
