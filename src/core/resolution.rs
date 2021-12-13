use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Resolution {
    Low,
    High,
}

impl Resolution {
    pub fn width(&self) -> u32 {
        match self {
            Resolution::Low => 320,
            Resolution::High => 640,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Resolution::Low => 180,
            Resolution::High => 320,
        }
    }

    pub fn total_pixels(&self) -> u32 {
        self.width() * self.height()
    }
}
