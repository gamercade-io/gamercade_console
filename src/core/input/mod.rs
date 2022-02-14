mod input_code;
mod input_state;
mod key_bindings;
mod local_input_manager;
mod player_input_entry;

pub use input_code::*;
pub use input_state::*;
pub use key_bindings::*;
pub use local_input_manager::*;
pub use player_input_entry::*;

#[derive(Debug)]
pub enum InputMode {
    Emulated,
    Gamepad,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Emulated
    }
}

pub trait AsApiCode: Sized {
    fn to_api_code(&self) -> u8;
    fn from_api_code(code: u8) -> Option<Self>;
}
