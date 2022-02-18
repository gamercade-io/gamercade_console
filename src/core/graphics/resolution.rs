use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Resolution {
    UltraLow,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    UltraHigh,
}

impl Resolution {
    pub fn width(&self) -> i32 {
        match self {
            Self::UltraLow => 128,
            Self::VeryLow => 160,
            Self::Low => 320,
            Self::Medium => 480,
            Self::High => 640,
            Self::VeryHigh => 1280,
            Self::UltraHigh => 1920,
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            Self::UltraLow => 72,
            Self::VeryLow => 90,
            Self::Low => 180,
            Self::Medium => 270,
            Self::High => 320,
            Self::VeryHigh => 720,
            Self::UltraHigh => 1080,
        }
    }

    pub fn total_pixels(&self) -> i32 {
        self.width() * self.height()
    }
}
