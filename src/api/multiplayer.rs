use super::i32_bool_to_option;
use crate::raw;

/// Returns the number of active players in the session.
pub fn num_players() -> usize {
    unsafe { raw::num_players() as usize }
}

/// Returns true if the requested player is local.
/// Returns None if the player_id is invalid.
/// WARNING: Using this incorrectly can desync clients!
/// Make sure you know what you're doing before using this.
pub fn is_local_player(player_id: usize) -> Option<bool> {
    let val = unsafe { raw::is_local_player(player_id as i32) };
    i32_bool_to_option(val)
}

/// Returns true if the requested player is remote.
/// Returns None if the player_id is invalid.
/// WARNING: Using this incorrectly can desync clients!
/// Make sure you know what you're doing before using this.
pub fn is_remote_player(player_id: usize) -> Option<bool> {
    let val = unsafe { raw::is_remote_player(player_id as i32) };
    i32_bool_to_option(val)
}
