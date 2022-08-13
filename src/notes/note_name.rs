use strum::{EnumCount, EnumIter};
use tinystr::TinyAsciiStr;

#[derive(Debug, Clone, Copy, EnumIter, EnumCount, PartialEq, Eq)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

impl NoteName {
    pub(crate) fn as_str(self) -> TinyAsciiStr<2> {
        TinyAsciiStr::from_str(match self {
            NoteName::A => "A ",
            NoteName::ASharp => "A#",
            NoteName::B => "B ",
            NoteName::C => "C ",
            NoteName::CSharp => "C#",
            NoteName::D => "D ",
            NoteName::DSharp => "D#",
            NoteName::E => "E ",
            NoteName::F => "F ",
            NoteName::FSharp => "F ",
            NoteName::G => "G ",
            NoteName::GSharp => "G#",
        })
        .unwrap()
    }
}
