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
    state: u8,
}

impl AnalogTrigger {
    const MASK: u8 = 0b0111_1111;

    pub fn to_f32(&self) -> f32 {
        (self.state & Self::MASK) as f32 / Self::MASK as f32
    }
}

#[derive(Clone, Debug)]
// 14 bits for with Analog Triggers
// 16 bits for binary triggers
pub struct Buttons {
    state: u16,
}

impl Buttons {
    pub fn get_button_a(&self) -> bool {
        self.state & 0b1 != 0
    }

    pub fn get_button_b(&self) -> bool {
        self.state & 0b10 != 0
    }

    pub fn get_button_c(&self) -> bool {
        self.state & 0b100 != 0
    }

    pub fn get_button_d(&self) -> bool {
        self.state & 0b1000 != 0
    }

    pub fn get_button_start(&self) -> bool {
        self.state & 0b1_0000 != 0
    }

    pub fn get_button_select(&self) -> bool {
        self.state & 0b10_0000 != 0
    }

    pub fn get_direction_up(&self) -> bool {
        self.state & 0b100_0000 != 0
    }

    pub fn get_direction_down(&self) -> bool {
        self.state & 0b1000_0000 != 0
    }

    pub fn get_direction_left(&self) -> bool {
        self.state & 0b1_0000_0000 != 0
    }

    pub fn get_direction_right(&self) -> bool {
        self.state & 0b10_0000_0000 != 0
    }

    pub fn get_button_left_shoulder(&self) -> bool {
        self.state & 0b100_0000_0000 != 0
    }

    pub fn get_button_right_shoulder(&self) -> bool {
        self.state & 0b1000_0000_0000 != 0
    }

    pub fn get_button_left_stick(&self) -> bool {
        self.state & 0b1_0000_0000_0000 != 0
    }

    pub fn get_button_right_stick(&self) -> bool {
        self.state & 0b10_0000_0000_0000 != 0
    }

    pub fn get_button_left_trigger(&self) -> bool {
        self.state & 0b100_0000_0000_0000 != 0
    }

    pub fn get_button_right_trigger(&self) -> bool {
        self.state & 0b1000_0000_0000_0000 != 0
    }
}

#[derive(Clone, Debug)]
// 4 bits
// Can be enabled/disabled for testing
pub struct DebugButtons {
    state: u8,
}

impl DebugButtons {
    const MASK: u8 = 0b1111;

    pub fn get_debug_button_1(&self) -> bool {
        self.state & 0b1 != 0
    }

    pub fn get_debug_button_2(&self) -> bool {
        self.state & 0b10 != 0
    }

    pub fn get_debug_button_3(&self) -> bool {
        self.state & 0b100 != 0
    }

    pub fn get_debug_button_4(&self) -> bool {
        self.state & 0b1000 != 0
    }
}
