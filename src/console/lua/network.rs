use std::net::SocketAddr;

use ggrs::Config;

use super::LuaConsole;
use crate::core::{InputState, PlayerInputEntry};

impl Config for LuaConsole {
    type Input = InputState;
    type State = Box<[PlayerInputEntry]>;
    type Address = SocketAddr;
}
