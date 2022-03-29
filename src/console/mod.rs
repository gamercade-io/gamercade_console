mod graphics_context;
mod input_context;
mod random_context;
mod wasm;

use ggrs::{Config, GGRSRequest};
pub use graphics_context::GraphicsContext;
pub use input_context::InputContext;
pub use random_context::RandomContext;
pub use wasm::WasmConsole;

use crate::core::Rom;

pub trait Console: Sized + Config {
    fn call_init(&mut self);
    fn call_update(&mut self);
    fn call_draw(&mut self);

    fn rom(&self) -> &Rom;

    fn blit(&self, buffer: &mut [u8]);

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>);
}
