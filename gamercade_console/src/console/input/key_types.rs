use gamercade_core::ButtonCode;

#[derive(Debug)]
pub(crate) enum KeyType {
    ButtonCode(ButtonCode),
    Analog(Analog),
    Trigger(AnalogSide),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Analog {
    pub(crate) side: AnalogSide,
    pub(crate) axis: AnalogAxis,
    pub(crate) direction: AnalogDirection,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum AnalogSide {
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum AnalogAxis {
    X,
    Y,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum AnalogDirection {
    Positive,
    Negative,
}
