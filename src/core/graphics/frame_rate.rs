use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FrameRate {
    SuperSlow,
    Slow,
    Normal,
    Fast,
}

impl FrameRate {
    pub fn frames_per_second(&self) -> usize {
        match self {
            FrameRate::SuperSlow => 8,
            FrameRate::Slow => 24,
            FrameRate::Normal => 30,
            FrameRate::Fast => 60,
        }
    }
}
