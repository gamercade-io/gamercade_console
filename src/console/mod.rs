mod draw_context;
mod lua_console;

pub use lua_console::LuaConsole;

use crate::core::Rom;

use self::draw_context::DrawContext;

pub trait Console {
    fn call_handle_input(&self);
    fn call_update(&self);
    fn call_draw(&self);

    fn rom(&self) -> &Rom;

    fn into_draw_context<'a>(&'a self, frame: &'a mut [u8]) -> DrawContext<'a> {
        DrawContext {
            rom: self.rom(),
            frame,
        }
    }
}
