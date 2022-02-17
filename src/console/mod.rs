mod graphics_context;
mod input_context;
mod lua;
mod wasm;

use ggrs::{Config, GGRSRequest};
pub use graphics_context::GraphicsContext;
pub use input_context::InputContext;
pub use lua::LuaConsole;
pub use wasm::WasmConsole;

use crate::core::Rom;

pub trait Console: Sized + Config {
    fn call_init(&self);
    fn call_update(&self);
    fn call_draw(&self);

    fn rom(&self) -> &Rom;

    fn blit(&self, buffer: &mut [u8]);

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>);
}
