use ggrs::PlayerType;

use crate::{api::MultiplayerApi, console::SessionDescriptor};

#[derive(Clone)]
pub struct MultiplayerContext {
    session: SessionDescriptor,
}

impl MultiplayerContext {
    pub fn new(session: SessionDescriptor) -> Self {
        Self { session }
    }
}

impl MultiplayerApi for MultiplayerContext {
    fn num_players(&self) -> i32 {
        self.session.num_players as i32
    }

    fn is_local_player(&self, player_id: i32) -> i32 {
        match self.session.player_types.get(player_id as usize) {
            Some(PlayerType::Local) => 1, //Target player is local
            Some(_) => 0,                 // Valid index, but not local
            None => -1,                   // Invalid index
        }
    }

    fn is_remote_player(&self, player_id: i32) -> i32 {
        match self.session.player_types.get(player_id as usize) {
            Some(PlayerType::Remote(_)) => 1, //Target player is remote
            Some(_) => 0,                     // Valid index, but not remote
            None => -1,                       // Invalid index
        }
    }
}
