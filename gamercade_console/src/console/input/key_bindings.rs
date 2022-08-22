use hashbrown::HashMap;
use winit::event::VirtualKeyCode;

use super::input_code::ButtonCode;

#[derive(Debug)]
pub struct KeyBindings {
    pub buttons: HashMap<VirtualKeyCode, ButtonCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        //TODO: Add other button/triggers/etc

        let buttons = [
            //Sticks
            (VirtualKeyCode::X, ButtonCode::LeftStick),
            (VirtualKeyCode::B, ButtonCode::RightStick),
            //Shoulders
            (VirtualKeyCode::E, ButtonCode::LeftShoulder),
            (VirtualKeyCode::Q, ButtonCode::LeftTrigger),
            (VirtualKeyCode::R, ButtonCode::RightShoulder),
            (VirtualKeyCode::Y, ButtonCode::RightTrigger),
            //DPad:
            (VirtualKeyCode::Up, ButtonCode::Up),
            (VirtualKeyCode::Down, ButtonCode::Down),
            (VirtualKeyCode::Left, ButtonCode::Left),
            (VirtualKeyCode::Right, ButtonCode::Right),
            //Buttons:
            (VirtualKeyCode::U, ButtonCode::A),
            (VirtualKeyCode::I, ButtonCode::B),
            (VirtualKeyCode::J, ButtonCode::C),
            (VirtualKeyCode::K, ButtonCode::D),
            (VirtualKeyCode::Key5, ButtonCode::Start),
            (VirtualKeyCode::Key6, ButtonCode::Select),
        ]
        .into_iter()
        .collect::<HashMap<VirtualKeyCode, ButtonCode>>();

        Self { buttons }
    }
}
