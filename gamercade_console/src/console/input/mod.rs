mod key_bindings;
mod local_input_manager;
mod player_input_entry;

pub use key_bindings::*;
pub use local_input_manager::*;
pub use player_input_entry::*;

#[derive(Debug)]
// TODO: Implement `Gamepad` later.
#[allow(dead_code)]
pub enum InputMode {
    Emulated,
    Gamepad,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Emulated
    }
}
