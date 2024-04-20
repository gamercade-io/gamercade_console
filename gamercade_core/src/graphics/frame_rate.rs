use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FrameRate {
    UltraSlow,
    VerySlow,
    Slow,
    Moderate,
    #[default]
    Normal,
    Fast,
    UltraFast,
}

impl FrameRate {
    pub const fn as_str(&self) -> &str {
        match self {
            FrameRate::UltraSlow => "Ultra Slow",
            FrameRate::VerySlow => "Very Slow",
            FrameRate::Slow => "Slow",
            FrameRate::Moderate => "Moderate",
            FrameRate::Normal => "Normal",
            FrameRate::Fast => "Fast",
            FrameRate::UltraFast => "Ultra Fast",
        }
    }

    pub const fn frames_per_second(self) -> usize {
        match self {
            FrameRate::UltraSlow => 12,
            FrameRate::VerySlow => 24,
            FrameRate::Slow => 30,
            FrameRate::Moderate => 48,
            FrameRate::Normal => 60,
            FrameRate::Fast => 120,
            FrameRate::UltraFast => 240,
        }
    }

    pub const fn default_input_delay(self) -> usize {
        match self {
            FrameRate::UltraSlow => 0,
            FrameRate::VerySlow => 0,
            FrameRate::Slow => 0,
            FrameRate::Moderate => 1,
            FrameRate::Normal => 1,
            FrameRate::Fast => 2,
            FrameRate::UltraFast => 3,
        }
    }

    pub fn frame_time(self) -> f32 {
        (self.frames_per_second() as f32).recip()
    }
}
