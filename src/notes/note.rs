use std::mem::MaybeUninit;

use serde::{Deserialize, Serialize};

use crate::{NoteName, TOTAL_NOTES_COUNT};

/// Newtype Note Id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteIndex(pub usize);

/// A representation of a musical note
#[derive(Debug, Clone)]
pub struct Note {
    pub octave: usize,
    pub name: NoteName,
    pub frequency: f32,
}

static mut NOTES_LUT: MaybeUninit<[Note; TOTAL_NOTES_COUNT]> = MaybeUninit::uninit();

/// Initializes the notes LUT
pub(crate) fn initialize_notes() {
    // TODO:
    println!("initialize_notes() is unimplemented.");
}

/// Get's a note for the given index
pub fn get_note(index: NoteIndex) -> &'static Note {
    unsafe {
        let notes = NOTES_LUT.assume_init_ref();
        &notes[index.0]
    }
}
