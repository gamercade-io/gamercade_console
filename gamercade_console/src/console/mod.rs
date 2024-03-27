mod bindings;
mod contexts;
mod input;
mod network;
mod wasm_console;

pub use contexts::Contexts;
use gamercade_fs::Rom;
use ggrs::{Config, GgrsRequest};
pub use input::*;
pub use network::{SessionDescriptor, WasmConsoleState};
pub use wasm_console::WasmConsole;

pub trait Console: Sized + Config {
    fn call_init(&mut self);
    fn call_update(&mut self);
    fn call_draw(&mut self);

    fn rom(&self) -> &Rom;

    fn blit(&self, buffer: &mut [u8]);

    fn handle_requests(&mut self, requests: Vec<GgrsRequest<Self>>);
}
