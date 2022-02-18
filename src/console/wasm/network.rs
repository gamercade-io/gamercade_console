use std::net::SocketAddr;

use ggrs::Config;
use wasmer::Instance;

use super::{Functions, WasmConsole};
use crate::core::{InputState, PlayerInputEntry};

#[derive(Clone)]
pub struct WasmConsoleState {
    inputs: Box<[PlayerInputEntry]>,
    wasm_state: Instance,
    functions: Functions,
}

impl Config for WasmConsole {
    type Input = InputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}
