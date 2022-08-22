mod note;
mod note_name;
mod octave;

pub use note::*;
pub use note_name::*;
pub use octave::*;

use strum::EnumCount;
pub const TOTAL_NOTES_COUNT: usize = (Octave::COUNT - 1) * NoteName::COUNT; //96 notes from C1 -> B9
