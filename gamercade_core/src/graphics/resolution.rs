use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Resolution {
    pub const ULTRALOW: (i32, i32) = (128, 72);
    pub const VERYLOW: (i32, i32) = (160, 90);
    pub const LOW: (i32, i32) = (320, 180);
    pub const MEDIUM: (i32, i32) = (480, 270);
    pub const HIGH: (i32, i32) = (640, 360);
    pub const VERYHIGH: (i32, i32) = (1280, 720);
    pub const ULTRAHIGH: (i32, i32) = (1920, 1080);

    pub const fn width(&self) -> i32 {
        match self {
            Resolution::UltraLow => Self::ULTRALOW.0,
            Resolution::VeryLow => Self::VERYLOW.0,
            Resolution::Low => Self::LOW.0,
            Resolution::Medium => Self::MEDIUM.0,
            Resolution::High => Self::HIGH.0,
            Resolution::VeryHigh => Self::VERYHIGH.0,
            Resolution::UltraHigh => Self::ULTRAHIGH.0,
        }
    }

    pub const fn height(&self) -> i32 {
        match self {
            Resolution::UltraLow => Self::ULTRALOW.1,
            Resolution::VeryLow => Self::VERYLOW.1,
            Resolution::Low => Self::LOW.1,
            Resolution::Medium => Self::MEDIUM.1,
            Resolution::High => Self::HIGH.1,
            Resolution::VeryHigh => Self::VERYHIGH.1,
            Resolution::UltraHigh => Self::ULTRAHIGH.1,
        }
    }

    pub const fn total_pixels(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn try_get_xcord<T: Into<i32>>(&self, value: T) -> Option<XCord> {
        let v: i32 = value.into();
        match 0 <= v && v < self.width() {
            true => Some(XCord(v as usize)),
            false => None,
        }
    }

    pub fn try_get_ycord<T: Into<i32>>(&self, value: T) -> Option<YCord> {
        let v: i32 = value.into();
        match 0 <= v && v < self.height() {
            true => Some(YCord(v as usize)),
            false => None,
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::Low
    }
}

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct XCord(usize);

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct YCord(usize);

impl XCord {
    pub fn try_for_screen<T: TryInto<i32>>(value: T, screen: &Resolution) -> Option<Self> {
        TryInto::try_into(value).map_or(None, |v| screen.try_get_xcord(v))
    }

    pub fn raw_value(&self) -> usize {
        self.0
    }
}

impl YCord {
    pub fn try_for_screen<T: TryInto<i32>>(value: T, screen: &Resolution) -> Option<Self> {
        TryInto::try_into(value).map_or(None, |v| screen.try_get_ycord(v))
    }

    pub fn raw_value(&self) -> usize {
        self.0
    }
}
