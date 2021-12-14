//TODO: This

#[derive(Clone, Debug)]
// 60-64 bits aka 8 bytes when compressed
pub struct InputState {
    left_stick: AnalogStick,
    right_stick: AnalogStick,
    left_trigger: AnalogTrigger,
    right_trigger: AnalogTrigger,
    buttons: Buttons,
    debug_buttons: DebugButtons,
}

#[derive(Clone, Debug)]
// 16 bits
pub struct AnalogStick {
    x_axis: i8,
    y_axis: i8,
}

#[derive(Clone, Debug)]
// 7 bits
// Sign bit will be dropped
pub struct AnalogTrigger {
    state: i8,
}

#[derive(Clone, Debug)]
// 14 bits for with Analog Triggers
// 16 bits for binary triggers
pub struct Buttons {
    state: u16,
}

#[derive(Clone, Debug)]
// 4 bits
// Can be enabled/disabled for testing
pub struct DebugButtons {
    state: u8,
}

impl DebugButtons {
    const MASK: u8 = 0b1111;

    pub fn get_button_1(&self) -> bool {
        self.state & 0b1 != 0
    }

    pub fn get_button_2(&self) -> bool {
        self.state & 0b10 != 0
    }

    pub fn get_button_3(&self) -> bool {
        self.state & 0b100 != 0
    }

    pub fn get_button_4(&self) -> bool {
        self.state & 0b1000 != 0
    }
}
