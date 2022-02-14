use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FrameRate {
    Slow,
    Normal,
    Fast,
}

impl FrameRate {
    pub fn frames_per_second(&self) -> usize {
        match self {
            FrameRate::Slow => 24,
            FrameRate::Normal => 30,
            FrameRate::Fast => 60,
        }
    }
}
