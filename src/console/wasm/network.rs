use std::net::SocketAddr;

use ggrs::Config;
use wasmer::Instance;

use super::WasmConsole;
use crate::core::{InputState, PlayerInputEntry};

#[derive(Clone)]
pub struct WasmConsoleState {
    inputs: Box<[PlayerInputEntry]>,
    wasm_state: Instance,
}

impl Config for WasmConsole {
    type Input = InputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}
