use gamercade_core::ButtonCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum KeyType {
    Button(ButtonCode),
    AnalogStick(Analog),
    Trigger(AnalogSide),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub(crate) struct Analog {
    pub(crate) side: AnalogSide,
    pub(crate) axis: AnalogAxis,
    pub(crate) direction: AnalogDirection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum AnalogSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum AnalogAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum AnalogDirection {
    Positive,
    Negative,
}
