mod graphics_context;
mod lua;

pub use graphics_context::GraphicsContext;
pub use lua::LuaConsole;

use crate::core::{InputState, Rom};

pub trait Console {
    fn call_init(&self);
    fn call_update(&self, input_states: &[InputState]);
    fn call_draw(&self);

    fn rom(&self) -> &Rom;

    fn blit(&self, buffer: &mut [u8]);
}
