use serde::{Deserialize, Serialize};

impl Default for SoundsData {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundsData {
    //TODO: This
//patches: Box<[Patch]>,
//sequences: Box<[Sequence]>

// How to handle samples?
}
