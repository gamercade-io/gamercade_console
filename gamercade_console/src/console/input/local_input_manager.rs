use super::{AnalogStick, AnalogTrigger, Buttons, InputMode, InputState, KeyBindings};

#[derive(Debug, Default)]
pub struct LocalInputManager {
    keybinds: KeyBindings,
    input_mode: InputMode,
}

impl LocalInputManager {
    pub fn generate_input_state(
        &self,
        helper: &winit_input_helper::WinitInputHelper,
    ) -> InputState {
        match self.input_mode {
            InputMode::Emulated => self.new_emulated_state(helper),
            InputMode::Gamepad => self.new_gamepad_state(),
        }
    }

    //TODO: This
    fn new_emulated_state(&self, helper: &winit_input_helper::WinitInputHelper) -> InputState {
        //TODO: Update analog sticks & triggers
        InputState {
            left_stick: AnalogStick::default(),
            right_stick: AnalogStick::default(),
            left_trigger: AnalogTrigger::default(),
            right_trigger: AnalogTrigger::default(),
            buttons: Buttons::generate_new(&self.keybinds, helper),
        }
    }

    //TODO: This
    fn new_gamepad_state(&self) -> InputState {
        todo!()
    }
}
