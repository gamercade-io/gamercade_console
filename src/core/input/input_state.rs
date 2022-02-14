//TODO: This
use super::{input_code::*, KeyBindings};

#[derive(Clone, Debug, Default)]
// 60-64 bits aka 8 bytes when compressed
pub struct InputState {
    pub left_stick: AnalogStick,
    pub right_stick: AnalogStick,
    pub left_trigger: AnalogTrigger,
    pub right_trigger: AnalogTrigger,
    pub buttons: Buttons,
}

#[derive(Clone, Debug, Default)]
// 16 bits
pub struct AnalogStick {
    x_axis: i8,
    y_axis: i8,
}

#[derive(Clone, Debug, Default)]
// 7 bits
// Sign bit will be dropped
pub struct AnalogTrigger {
    state: u8,
}

impl AnalogTrigger {
    const MASK: u8 = 0b0111_1111;

    pub fn to_f32(&self) -> f32 {
        (self.state & Self::MASK) as f32 / Self::MASK as f32
    }
}

#[derive(Clone, Debug, Default)]
// 14 bits for with Analog Triggers
// 16 bits for binary triggers
pub struct Buttons {
    state: u16,
}

impl Buttons {
    pub fn enable_button(&mut self, code: ButtonCode) {
        self.state |= code.to_bit_mask();
    }

    pub fn get_button_state(&self, code: ButtonCode) -> bool {
        self.state & code.to_bit_mask() != 0
    }

    pub fn generate_new(
        binds: &KeyBindings,
        input_helper: &winit_input_helper::WinitInputHelper,
    ) -> Self {
        let mut output = Buttons::default();

        binds.buttons.iter().for_each(|(code, input)| {
            if input_helper.key_held(*code) {
                output.enable_button(*input)
            }
        });

        output
    }
}
