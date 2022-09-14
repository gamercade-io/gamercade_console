use gamercade_core::{AnalogStick, AnalogTrigger, Buttons, InputState};

use super::{InputMode, KeyBindings};

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
            buttons: generate_new_buttons(&self.keybinds, helper),
        }
    }

    //TODO: This
    fn new_gamepad_state(&self) -> InputState {
        todo!()
    }
}

fn generate_new_buttons(
    binds: &KeyBindings,
    input_helper: &winit_input_helper::WinitInputHelper,
) -> Buttons {
    let mut output = Buttons::default();

    binds.buttons.iter().for_each(|(code, input)| {
        if input_helper.key_held(*code) {
            output.enable_button(*input)
        }
    });

    output
}
