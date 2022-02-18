use std::net::SocketAddr;

use ggrs::Config;
use wasmer::Instance;

use super::{Functions, WasmConsole};
use crate::core::{InputState, PlayerInputEntry};

pub struct WasmConsoleState {
    pub(crate) input_state: Box<[PlayerInputEntry]>,
    pub(crate) instance: Instance,
    pub(crate) functions: Functions,
}

impl Clone for WasmConsoleState {
    fn clone(&self) -> Self {
        let new_instance = self.instance.clone();
        let functions = Functions::find_functions(&new_instance);
        Self {
            input_state: self.input_state.clone(),
            instance: new_instance,
            functions,
        }
    }
}

impl Config for WasmConsole {
    type Input = InputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}
