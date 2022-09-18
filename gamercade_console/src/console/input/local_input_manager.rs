use gamercade_core::{ButtonCode, InputState};
use gilrs::{Axis, Button, Gamepad, GamepadId, Gilrs};

use super::{
    gamepad_bindings::GamepadBindings,
    key_types::{AnalogSide, KeyType},
    InputMode, KeyBindings,
};

#[derive(Debug)]
pub struct LocalInputManager {
    keybinds: KeyBindings,
    gamepad_binds: GamepadBindings,
    pub(crate) input_mode: InputMode,
}

impl LocalInputManager {
    pub fn new(input_mode: InputMode) -> Self {
        Self {
            keybinds: KeyBindings::default(),
            gamepad_binds: GamepadBindings::default(),
            input_mode,
        }
    }

    pub fn generate_input_state(
        &self,
        helper: &winit_input_helper::WinitInputHelper,
        gilrs: &Gilrs,
    ) -> InputState {
        match self.input_mode {
            InputMode::Emulated => self.new_emulated_state(helper),
            InputMode::Gamepad(id) => self.new_gamepad_state(id, gilrs),
        }
    }

    fn new_emulated_state(&self, helper: &winit_input_helper::WinitInputHelper) -> InputState {
        generate_emulated_state(&self.keybinds, helper)
    }

    //TODO: This
    fn new_gamepad_state(&self, id: GamepadId, gilrs: &Gilrs) -> InputState {
        if let Some(gamepad) = gilrs.connected_gamepad(id) {
            generate_gamepad_state(&self.gamepad_binds, &gamepad)
        } else {
            InputState::default()
        }
    }
}

fn generate_gamepad_state(binds: &GamepadBindings, gamepad: &Gamepad) -> InputState {
    let mut output = InputState::default();

    binds.buttons.iter().for_each(|(button, input)| {
        if gamepad.is_pressed(*button) {
            output.buttons.enable_button(*input);
        }
    });

    if let Some(axis) = gamepad.axis_data(Axis::LeftStickX) {
        output.left_stick.set_x_axis(axis.value())
    }
    if let Some(axis) = gamepad.axis_data(Axis::LeftStickY) {
        output.left_stick.set_y_axis(axis.value())
    }

    if let Some(axis) = gamepad.axis_data(Axis::RightStickX) {
        output.right_stick.set_x_axis(axis.value())
    }
    if let Some(axis) = gamepad.axis_data(Axis::RightStickY) {
        output.right_stick.set_y_axis(axis.value())
    }

    if let Some(trigger) = gamepad.button_data(Button::LeftTrigger2) {
        output.left_trigger.set_value(trigger.value())
    }
    if let Some(trigger) = gamepad.button_data(Button::RightTrigger2) {
        output.right_trigger.set_value(trigger.value())
    }

    output
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
                KeyType::Analog(emulated) => emulated.adjust_input_state(&mut output),
                KeyType::Trigger(side) => match side {
                    AnalogSide::Left => {
                        output.buttons.enable_button(ButtonCode::LeftTrigger);
                        output.left_trigger.set_value(1.0);
                    }
                    AnalogSide::Right => {
                        output.buttons.enable_button(ButtonCode::RightTrigger);
                        output.right_trigger.set_value(1.0)
                    }
                },
            }
        }
    });

    output
}
