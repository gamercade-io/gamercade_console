use std::net::SocketAddr;

use bytemuck::{Pod, Zeroable};
use gamercade_core::{Buttons, InputState, MouseState};
use gamercade_interface::{NetworkSession, NetworkedPlayerType};
use gamercade_sound_engine::SoundEngineData;
use ggrs::{Config, PlayerType};

use super::WasmConsole;

#[derive(Clone)]
pub struct WasmConsoleState {
    pub(crate) previous_buttons: Box<[Buttons]>,
    pub(crate) before_dp: Vec<u8>,
    pub(crate) after_dp: Vec<u8>,
    pub(crate) sound_engine_data: SoundEngineData,
}

#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

impl Config for WasmConsole {
    type Input = NetworkInputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}

impl SessionDescriptor {
    pub fn new(port: u16, network_session: &NetworkSession) -> Self {
        let mut num_players = 0;

        let player_types: Vec<_> = network_session
            .players
            .iter()
            .flat_map(|client| {
                let client_type = match client.kind {
                    NetworkedPlayerType::Local => PlayerType::Local,
                    NetworkedPlayerType::Remote(addr) => PlayerType::Remote(addr),
                };

                num_players += client.count;
                std::iter::repeat(client_type).take(client.count)
            })
            .collect();

        Self {
            num_players,
            player_types: player_types.into_boxed_slice(),
            port,
        }
    }
}
