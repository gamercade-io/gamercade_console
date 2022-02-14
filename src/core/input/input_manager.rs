use super::{InputState, PlayerInputEntry};

pub struct InputManager {
    player_inputs: Vec<PlayerInputEntry>,
}

impl InputManager {
    fn push_input_state(&mut self, index: usize, next_state: InputState) {
        self.player_inputs[index].push_input_state(next_state);
    }
}
