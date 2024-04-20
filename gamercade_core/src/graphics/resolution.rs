use serde::{Deserialize, Serialize};

const fn standard_resolution(size: ResolutionSize) -> (i32, i32) {
    match size {
        ResolutionSize::UltraLow => (128, 72),
        ResolutionSize::VeryLow => (160, 90),
        ResolutionSize::Low => (320, 180),
        ResolutionSize::Medium => (480, 270),
        ResolutionSize::High => (640, 360),
        ResolutionSize::VeryHigh => (1280, 720),
        ResolutionSize::UltraHigh => (1920, 1080),
    }
}

const fn square_resolution(size: ResolutionSize) -> (i32, i32) {
    match size {
        ResolutionSize::UltraLow => (64, 64),
        ResolutionSize::VeryLow => (128, 128),
        ResolutionSize::Low => (256, 256),
        ResolutionSize::Medium => (384, 384),
        ResolutionSize::High => (512, 512),
        ResolutionSize::VeryHigh => (768, 768),
        ResolutionSize::UltraHigh => (1024, 1024),
    }
}

const fn classic_resolution(size: ResolutionSize) -> (i32, i32) {
    match size {
        ResolutionSize::UltraLow => (120, 90),
        ResolutionSize::VeryLow => (160, 120),
        ResolutionSize::Low => (240, 180),
        ResolutionSize::Medium => (360, 270),
        ResolutionSize::High => (480, 360),
        ResolutionSize::VeryHigh => (960, 720),
        ResolutionSize::UltraHigh => (1440, 1080),
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolutionSize {
    UltraLow,
    VeryLow,
    Low,
    #[default]
    Medium,
    High,
    VeryHigh,
    UltraHigh,
}
impl ResolutionSize {
    pub const fn as_str(&self) -> &str {
        match self {
            ResolutionSize::UltraLow => "Ultra Low",
            ResolutionSize::VeryLow => "Very Low",
            ResolutionSize::Low => "Low",
            ResolutionSize::Medium => "Medium",
            ResolutionSize::High => "High",
            ResolutionSize::VeryHigh => "Very High",
            ResolutionSize::UltraHigh => "Ultra High",
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolutionRatio {
    #[default]
    Standard,
    Square,
    Classic,
}

impl ResolutionRatio {
    pub const fn as_str(&self) -> &str {
        match self {
            ResolutionRatio::Standard => "Standard (16:9)",
            ResolutionRatio::Square => "Square (1:1)",
            ResolutionRatio::Classic => "Classic (4:3)",
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub size: ResolutionSize,
    pub ratio: ResolutionRatio,
}

impl Resolution {
    pub const fn width(&self) -> i32 {
        self.ratio_helper().0
    }

    pub const fn height(&self) -> i32 {
        self.ratio_helper().1
    }

    const fn ratio_helper(&self) -> (i32, i32) {
        match self.ratio {
            ResolutionRatio::Standard => standard_resolution(self.size),
            ResolutionRatio::Square => square_resolution(self.size),
            ResolutionRatio::Classic => classic_resolution(self.size),
        }
    }

    pub const fn total_pixels(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn try_get_xcord<T: Into<i32>>(&self, value: T) -> Option<XCord> {
        let v = value.into();
        match 0 <= v && v < self.width() {
            true => Some(XCord(v as usize)),
            false => None,
        }
    }

    pub fn try_get_ycord<T: Into<i32>>(&self, value: T) -> Option<YCord> {
        let v = value.into();
        match 0 <= v && v < self.height() {
            true => Some(YCord(v as usize)),
            false => None,
        }
    }
}

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct XCord(usize);

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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
