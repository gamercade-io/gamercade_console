use super::{input_code::*, KeyBindings};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Pod, Zeroable)]
// 60-64 bits aka 8 bytes when compressed
pub struct InputState {
    pub left_stick: AnalogStick,
    pub right_stick: AnalogStick,
    pub left_trigger: AnalogTrigger,
    pub right_trigger: AnalogTrigger,
    pub buttons: Buttons,
}

impl InputState {
    pub fn as_raw_state(&self) -> i64 {
        todo!()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Pod, Zeroable)]
// 16 bits
pub struct AnalogStick {
    x_axis: i8,
    y_axis: i8,
}

impl AnalogStick {
    pub fn set_x_axis(&mut self, value: f32) {
        assert!(value <= 1.0);
        assert!(value >= -1.0);
        self.x_axis = (value * i8::MAX as f32) as i8;
    }

    pub fn set_y_axis(&mut self, value: f32) {
        assert!(value <= 1.0);
        assert!(value >= -1.0);
        self.y_axis = (value * i8::MAX as f32) as i8;
    }

    pub fn get_x_axis(&self) -> f32 {
        (self.x_axis / i8::MAX) as f32
    }

    pub fn get_y_axis(&self) -> f32 {
        (self.y_axis / i8::MAX) as f32
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Pod, Zeroable)]
// 7 bits
// Sign bit will be dropped
pub struct AnalogTrigger {
    state: u8,
}

impl AnalogTrigger {
    const MASK: u8 = 0b0111_1111;

    pub fn get_value(&self) -> f32 {
        (self.state & Self::MASK) as f32 / Self::MASK as f32
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Pod, Zeroable)]
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
