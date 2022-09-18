use gamercade_core::InputState;

use super::{InputMode, KeyBindings, KeyType};

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
        generate_emulated_state(&self.keybinds, helper)
    }

    //TODO: This
    fn new_gamepad_state(&self) -> InputState {
        todo!()
    }
}

fn generate_emulated_state(
    binds: &KeyBindings,
    input_helper: &winit_input_helper::WinitInputHelper,
) -> InputState {
    let mut output = InputState::default();

    binds.buttons.iter().for_each(|(code, input)| {
        if input_helper.key_held(*code) {
            match input {
                KeyType::ButtonCode(code) => output.buttons.enable_button(*code),
                KeyType::Emulated(emulated) => emulated.adjust_input_state(&mut output),
            }
        }
    });

    output
}
