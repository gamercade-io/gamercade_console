use std::mem::MaybeUninit;

use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use tinystr::TinyAsciiStr;

use crate::{NoteName, Octave, TOTAL_NOTES_COUNT};

/// Newtype Note Id
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NoteId(pub usize);

/// A representation of a musical note
#[derive(Debug, Clone)]
pub struct Note {
    pub name: TinyAsciiStr<3>,
    pub frequency: f32,
}

static mut NOTES_LUT: MaybeUninit<[Note; TOTAL_NOTES_COUNT]> = MaybeUninit::uninit();
const FIRST_NOTE_OFFSET: usize = 2;

/// Initializes the notes LUT
pub fn initialize_notes() {
    let mut octave_iter = Octave::iter().peekable(); //Start at 1
    let mut name_iter = NoteName::iter().cycle(); // Start at A

    name_iter.nth(FIRST_NOTE_OFFSET); // Advance to C1

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
pub fn get_note(index: NoteId) -> &'static Note {
    unsafe {
        let notes = NOTES_LUT.assume_init_ref();
        &notes[index.0]
    }
}

/// Converts a note index to a frequency, based on how far from A4 it is
fn note_to_frequency(offset: isize) -> f32 {
    440.0 * 2.0_f32.powf((offset - 69) as f32 / 12.0)
}

pub fn name_octave_to_index(name: NoteName, octave: Octave) -> Option<NoteId> {
    let offset = name.as_index_offset();

    let too_low = Octave::One == octave && offset <= FIRST_NOTE_OFFSET;
    let too_high = Octave::Nine == octave && offset > FIRST_NOTE_OFFSET;

    if too_low || too_high {
        None
    } else {
        Some(NoteId(
            (octave.as_index_multiplier() * NoteName::COUNT) + offset - FIRST_NOTE_OFFSET - 1,
        ))
    }
}

pub fn from_name_octave(name: NoteName, octave: Octave) -> Option<&'static Note> {
    name_octave_to_index(name, octave).map(get_note)
}

#[cfg(test)]
mod tests {
    use crate::{
        from_name_octave, get_note, initialize_notes, NoteId, NoteName, Octave, TOTAL_NOTES_COUNT,
    };

    #[test]
    fn test_from_name_octave() {
        initialize_notes();

        let first = get_note(NoteId(0)).name;
        let second = get_note(NoteId(1)).name;
        let c3 = get_note(NoteId(24)).name;
        let last = get_note(NoteId(TOTAL_NOTES_COUNT - 1)).name;
        let a4 = get_note(NoteId(33)).name;

        let test_first = from_name_octave(NoteName::C, Octave::One).unwrap().name;
        let test_second = from_name_octave(NoteName::CSharp, Octave::One)
            .unwrap()
            .name;
        let test_c3 = from_name_octave(NoteName::C, Octave::Three).unwrap().name;
        let test_last = from_name_octave(NoteName::B, Octave::Nine).unwrap().name;
        let test_a4 = from_name_octave(NoteName::A, Octave::Four).unwrap().name;

        assert_eq!(first, test_first);
        assert_eq!(second, test_second);
        assert_eq!(c3, test_c3);
        assert_eq!(last, test_last);
        assert_eq!(a4, test_a4);
        assert!(from_name_octave(NoteName::B, Octave::One).is_none());
        assert!(from_name_octave(NoteName::C, Octave::Nine).is_none());
    }
}
