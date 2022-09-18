use gamercade_core::{ButtonCode, InputState};

use hashbrown::HashMap;
use winit::event::VirtualKeyCode;

use super::key_types::{Analog, AnalogAxis, AnalogDirection, AnalogSide, KeyType};

impl Analog {
    pub(crate) fn adjust_input_state(self, input_state: &mut InputState) {
        let value = match self.direction {
            AnalogDirection::Positive => 1.0,
            AnalogDirection::Negative => -1.0,
        };

        let stick = match self.side {
            AnalogSide::Left => &mut input_state.left_stick,
            AnalogSide::Right => &mut input_state.right_stick,
        };

        match self.axis {
            AnalogAxis::X => stick.set_x_axis(value),
            AnalogAxis::Y => stick.set_y_axis(value),
        };
    }
}

#[derive(Debug)]
pub(crate) struct KeyBindings {
    pub buttons: HashMap<VirtualKeyCode, KeyType>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let buttons = [
            //Sticks
            (
                VirtualKeyCode::X,
                KeyType::ButtonCode(ButtonCode::LeftStick),
            ),
            (
                VirtualKeyCode::B,
                KeyType::ButtonCode(ButtonCode::RightStick),
            ),
            //Shoulders
            (
                VirtualKeyCode::E,
                KeyType::ButtonCode(ButtonCode::LeftShoulder),
            ),
            (VirtualKeyCode::Q, KeyType::Trigger(AnalogSide::Left)),
            (
                VirtualKeyCode::R,
                KeyType::ButtonCode(ButtonCode::RightShoulder),
            ),
            (VirtualKeyCode::Y, KeyType::Trigger(AnalogSide::Right)),
            //DPad:
            (VirtualKeyCode::Up, KeyType::ButtonCode(ButtonCode::Up)),
            (VirtualKeyCode::Down, KeyType::ButtonCode(ButtonCode::Down)),
            (VirtualKeyCode::Left, KeyType::ButtonCode(ButtonCode::Left)),
            (
                VirtualKeyCode::Right,
                KeyType::ButtonCode(ButtonCode::Right),
            ),
            //Buttons:
            (VirtualKeyCode::U, KeyType::ButtonCode(ButtonCode::A)),
            (VirtualKeyCode::I, KeyType::ButtonCode(ButtonCode::B)),
            (VirtualKeyCode::J, KeyType::ButtonCode(ButtonCode::C)),
            (VirtualKeyCode::K, KeyType::ButtonCode(ButtonCode::D)),
            (VirtualKeyCode::Key5, KeyType::ButtonCode(ButtonCode::Start)),
            (
                VirtualKeyCode::Key6,
                KeyType::ButtonCode(ButtonCode::Select),
            ),
            //Left Stick Axis
            (
                VirtualKeyCode::W,
                KeyType::Analog(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Positive,
                }),
            ),
            (
                VirtualKeyCode::S,
                KeyType::Analog(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::A,
                KeyType::Analog(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::D,
                KeyType::Analog(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Positive,
                }),
            ),
            //Right Stick Axis,
            (
                VirtualKeyCode::T,
                KeyType::Analog(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Positive,
                }),
            ),
            (
                VirtualKeyCode::G,
                KeyType::Analog(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::F,
                KeyType::Analog(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::H,
                KeyType::Analog(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Positive,
                }),
            ),
        ]
        .into_iter()
        .collect::<HashMap<VirtualKeyCode, KeyType>>();

        Self { buttons }
    }
}
