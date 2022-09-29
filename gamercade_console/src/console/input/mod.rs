mod gamepad_bindings;
mod key_bindings;
mod key_types;
mod local_input_manager;
mod player_input_entry;

use gilrs::GamepadId;
use key_bindings::*;
pub use local_input_manager::*;
pub use player_input_entry::*;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InputMode {
    Emulated,
    Gamepad(GamepadId),
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Emulated
    }
}

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct LocalControllerId(pub usize);
