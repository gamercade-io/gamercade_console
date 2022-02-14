use super::{Buttons, InputState};

#[derive(Debug, Default)]
pub struct PlayerInputEntry {
    pub(crate) previous: Buttons,
    pub(crate) current: InputState,
}

impl PlayerInputEntry {
    pub fn push_input_state(&mut self, next_state: InputState) {
        self.previous = self.current.buttons.clone();
        self.current = next_state;
    }
}
