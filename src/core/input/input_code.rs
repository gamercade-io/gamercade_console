use strum::EnumIter;

//TODO: Analog sticks, analog triggers, etc
#[derive(Debug, Copy, Clone)]
pub enum InputCode {
    AnalogStick(AnalogSideCode, AxisCode),
    AnalogTrigger(AnalogSideCode),
    Button(ButtonCode),
}

// For AnalogSticks
#[derive(Debug, Copy, Clone)]
pub enum AxisCode {
    X,
    Y,
}

// For Left/Right Sticks/Triggers
#[derive(Debug, Copy, Clone)]
pub enum AnalogSideCode {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ButtonCode {
    // DPad
    Up,
    Down,
    Left,
    Right,

    // Buttons
    A,
    B,
    C,
    D,
    Start,
    Select,
    LeftShoulder,
    RightShoulder,
    LeftStick,
    RightStick,

    // Emulated
    LeftTrigger,
    RightTrigger,
}

impl ToBitMask<u16> for ButtonCode {
    fn to_bit_mask(&self) -> u16 {
        match self {
            Self::Up => 0b100_0000,
            Self::Down => 0b1000_0000,
            Self::Left => 0b1_0000_0000,
            Self::Right => 0b10_0000_0000,
            Self::A => 0b1,
            Self::B => 0b10,
            Self::C => 0b100,
            Self::D => 0b1000,
            Self::Start => 0b1_0000,
            Self::Select => 0b10_0000,
            Self::LeftShoulder => 0b100_0000_0000,
            Self::RightShoulder => 0b1000_0000_0000,
            Self::LeftStick => 0b1_0000_0000_0000,
            Self::RightStick => 0b10_0000_0000_0000,
            Self::LeftTrigger => 0b100_0000_0000_0000,
            Self::RightTrigger => 0b1000_0000_0000_0000,
        }
    }
}

pub trait ToBitMask<T> {
    fn to_bit_mask(&self) -> T;
}
