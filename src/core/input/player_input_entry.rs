use super::InputState;

pub struct PlayerInputEntry {
    previous: InputState,
    current: InputState,
}

impl PlayerInputEntry {
    pub fn push_input_state(&mut self, next_state: InputState) {
        drop(std::mem::replace(
            &mut self.previous,
            std::mem::replace(&mut self.current, next_state),
        ));
    }
}
