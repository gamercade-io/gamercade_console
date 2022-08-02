use crate::{i32_bool_to_option, raw};

pub fn num_players() -> usize {
    unsafe { raw::num_players() as usize }
}

// WARNING: Using this is possible to desync clients!
// Make sure you know what you're doing before using these.
pub fn is_local_player(player_id: usize) -> Option<bool> {
    let val = unsafe { raw::is_local_player(player_id as i32) };
    i32_bool_to_option(val)
}
pub fn is_remote_player(player_id: usize) -> Option<bool> {
    let val = unsafe { raw::is_remote_player(player_id as i32) };
    i32_bool_to_option(val)
}
