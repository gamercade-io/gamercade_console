use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FrameRate {
    SuperSlow,
    Slow,
    Normal,
    Fast,
    SuperFast,
}

impl FrameRate {
    pub fn frames_per_second(&self) -> usize {
        match self {
            FrameRate::SuperSlow => 24,
            FrameRate::Slow => 30,
            FrameRate::Normal => 60,
            FrameRate::Fast => 120,
            FrameRate::SuperFast => 240,
        }
    }
}
