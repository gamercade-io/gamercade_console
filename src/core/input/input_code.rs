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

#[derive(Debug, Copy, Clone)]
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

impl ButtonCode {
    const S_UP: &'static str = "up";
    const S_DOWN: &'static str = "down";
    const S_LEFT: &'static str = "left";
    const S_RIGHT: &'static str = "right";
    const S_A: &'static str = "a";
    const S_B: &'static str = "b";
    const S_C: &'static str = "c";
    const S_D: &'static str = "d";
    const S_START: &'static str = "start";
    const S_SELECT: &'static str = "select";
    const S_LEFT_SHOULDER: &'static str = "lshoulder";
    const S_RIGHT_SHOULDER: &'static str = "rshoulder";
    const S_LEFT_STICK: &'static str = "rstick";
    const S_RIGHT_STICK: &'static str = "lstick";
    const S_LEFT_TRIGGER: &'static str = "ltrigger";
    const S_RIGHT_TRIGGER: &'static str = "rtrigger";
}

impl IntoBitMask<u16> for ButtonCode {
    fn into_bit_mask(&self) -> u16 {
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

impl LuaCode for ButtonCode {
    fn into_lua_code(&self) -> &str {
        match self {
            Self::Up => Self::S_UP,
            Self::Down => Self::S_DOWN,
            Self::Left => Self::S_LEFT,
            Self::Right => Self::S_RIGHT,
            Self::A => Self::S_A,
            Self::B => Self::S_B,
            Self::C => Self::S_C,
            Self::D => Self::S_D,
            Self::Start => Self::S_START,
            Self::Select => Self::S_SELECT,
            Self::LeftShoulder => Self::S_LEFT_SHOULDER,
            Self::RightShoulder => Self::S_RIGHT_SHOULDER,
            Self::LeftStick => Self::S_LEFT_STICK,
            Self::RightStick => Self::S_RIGHT_STICK,
            Self::LeftTrigger => Self::S_LEFT_TRIGGER,
            Self::RightTrigger => Self::S_RIGHT_TRIGGER,
        }
    }

    fn from_lua_code(str: &str) -> Option<Self> {
        match str.to_lowercase().as_str() {
            Self::S_UP => Some(Self::Up),
            Self::S_DOWN => Some(Self::Down),
            Self::S_LEFT => Some(Self::Left),
            Self::S_RIGHT => Some(Self::Right),
            Self::S_A => Some(Self::A),
            Self::S_B => Some(Self::B),
            Self::S_C => Some(Self::C),
            Self::S_D => Some(Self::D),
            Self::S_START => Some(Self::Start),
            Self::S_SELECT => Some(Self::Select),
            Self::S_LEFT_SHOULDER => Some(Self::LeftShoulder),
            Self::S_RIGHT_SHOULDER => Some(Self::RightShoulder),
            Self::S_LEFT_STICK => Some(Self::LeftStick),
            Self::S_RIGHT_STICK => Some(Self::RightStick),
            Self::S_LEFT_TRIGGER => Some(Self::LeftTrigger),
            Self::S_RIGHT_TRIGGER => Some(Self::RightTrigger),
            _ => None,
        }
    }
}

pub trait IntoBitMask<T> {
    fn into_bit_mask(&self) -> T;
}

pub trait LuaCode: Sized {
    fn into_lua_code(&self) -> &str;
    fn from_lua_code(str: &str) -> Option<Self>;
}
