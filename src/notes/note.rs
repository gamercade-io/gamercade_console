use std::mem::MaybeUninit;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use tinystr::TinyAsciiStr;

use crate::{NoteName, Octave, TOTAL_NOTES_COUNT};

/// Newtype Note Id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteIndex(pub usize);

/// A representation of a musical note
#[derive(Debug, Clone)]
pub struct Note {
    pub name: TinyAsciiStr<3>,
    pub frequency: f32,
}

static mut NOTES_LUT: MaybeUninit<[Note; TOTAL_NOTES_COUNT]> = MaybeUninit::uninit();

/// Initializes the notes LUT
pub(crate) fn initialize_notes() {
    let mut octave_iter = Octave::iter().peekable(); //Start at 1
    let mut name_iter = NoteName::iter().cycle(); // Start at A

    name_iter.nth(2); // Advance to C1

    unsafe {
        NOTES_LUT.write(std::array::from_fn(|index| {
            // C1 is 45 notes away from A4. (69 - 45 = 24)
            let index = index + 24;

            let name = name_iter.next().unwrap();

            if name == NoteName::A {
                octave_iter.next();
            }

            let octave = octave_iter.peek().unwrap().as_str();

            let name = TinyAsciiStr::from_str(&[name.as_str().as_str(), octave.as_str()].concat())
                .unwrap();
            let frequency = note_to_frequency(index as isize);

            Note { name, frequency }
        }));
    }
}

/// Get's a note for the given index
pub fn get_note(index: NoteIndex) -> &'static Note {
    unsafe {
        let notes = NOTES_LUT.assume_init_ref();
        &notes[index.0]
    }
}

/// Converts a note index to a frequency, based on how far from A4 it is
fn note_to_frequency(offset: isize) -> f32 {
    440.0 * 2.0_f32.powf((offset - 69) as f32 / 12.0)
}
