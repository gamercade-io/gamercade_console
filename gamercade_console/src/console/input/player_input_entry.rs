use super::{Buttons, InputState};

// TOOD: Move this to SoA instead of AoS for perf?
#[derive(Debug, Default, Clone)]
pub struct PlayerInputEntry {
    pub(crate) previous: Buttons,
    pub(crate) current: InputState,
}
