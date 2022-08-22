use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Resolution {
    UltraLow,  // 128 x 72
    VeryLow,   // 160 x 90
    Low,       // 320 x 180
    Medium,    // 480 x 270
    High,      // 640 x 360
    VeryHigh,  // 1280 x 720
    UltraHigh, // 1920 x 1080
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Low
    }
}

impl Resolution {
    pub fn width(self) -> i32 {
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

    pub fn height(self) -> i32 {
        match self {
            Self::UltraLow => 72,
            Self::VeryLow => 90,
            Self::Low => 180,
            Self::Medium => 270,
            Self::High => 360,
            Self::VeryHigh => 720,
            Self::UltraHigh => 1080,
        }
    }

    pub fn total_pixels(self) -> i32 {
        self.width() * self.height()
    }
}
