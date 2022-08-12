mod note;
mod note_name;

pub use note::*;
pub use note_name::*;

pub const NOTE_NAME_COUNT: usize = 12; //A to G#
pub const OCTAVE_MIN: usize = 1;
pub const OCTAVE_MAX: usize = 8;
pub const TOTAL_NOTES_COUNT: usize = ((OCTAVE_MAX - OCTAVE_MIN) + 1) * NOTE_NAME_COUNT; //96 notes from C1 -> B8
