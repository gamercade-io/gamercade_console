mod input_code;
mod input_state;

pub use input_code::*;
pub use input_state::*;

pub trait AsApiCode: Sized {
    fn to_api_code(&self) -> u8;
    fn from_api_code(code: u8) -> Option<Self>;
}
