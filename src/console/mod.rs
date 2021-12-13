mod graphics_context;
mod lua_console;

pub use lua_console::LuaConsole;

use crate::core::Rom;

pub trait Console {
    fn call_input(&self, button_pressed: bool);
    fn call_update(&self);
    fn call_draw(&self);

    fn rom(&self) -> &Rom;

    fn blit(&self, buffer: &mut [u8]);
}
