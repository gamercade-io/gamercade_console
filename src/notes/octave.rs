use strum::{EnumCount, EnumIter};
use tinystr::TinyAsciiStr;

#[derive(Debug, Clone, Copy, EnumIter, EnumCount)]
pub enum Octave {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Octave {
    pub(crate) fn as_str(self) -> TinyAsciiStr<1> {
        TinyAsciiStr::from_str(match self {
            Octave::One => "1",
            Octave::Two => "2",
            Octave::Three => "3",
            Octave::Four => "4",
            Octave::Five => "5",
            Octave::Six => "6",
            Octave::Seven => "7",
            Octave::Eight => "8",
            Octave::Nine => "9",
        })
        .unwrap()
    }
}
