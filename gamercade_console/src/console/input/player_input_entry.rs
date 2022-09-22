use gamercade_core::{Buttons, InputState, MouseState};

// TOOD: Move this to SoA instead of AoS for perf?
#[derive(Debug, Default, Clone)]
pub struct PlayerInputEntry {
    pub(crate) previous: Buttons,
    pub(crate) current: InputState,

    pub(crate) previous_mouse: MouseState,
    pub(crate) current_mouse: MouseState,
}
