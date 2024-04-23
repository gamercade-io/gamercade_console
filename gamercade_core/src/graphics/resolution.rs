use serde::{Deserialize, Serialize};

const fn standard_resolution(size: Size) -> (i32, i32) {
    match size {
        Size::UltraLow => (128, 72),
        Size::VeryLow => (160, 90),
        Size::Low => (320, 180),
        Size::Medium => (480, 270),
        Size::High => (640, 360),
        Size::VeryHigh => (1280, 720),
        Size::UltraHigh => (1920, 1080),
    }
}

const fn square_resolution(size: Size) -> (i32, i32) {
    match size {
        Size::UltraLow => (64, 64),
        Size::VeryLow => (128, 128),
        Size::Low => (256, 256),
        Size::Medium => (384, 384),
        Size::High => (512, 512),
        Size::VeryHigh => (768, 768),
        Size::UltraHigh => (1024, 1024),
    }
}

const fn classic_resolution(size: Size) -> (i32, i32) {
    match size {
        Size::UltraLow => (120, 90),
        Size::VeryLow => (160, 120),
        Size::Low => (240, 180),
        Size::Medium => (360, 270),
        Size::High => (480, 360),
        Size::VeryHigh => (960, 720),
        Size::UltraHigh => (1440, 1080),
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    UltraLow,
    VeryLow,
    Low,
    #[default]
    Medium,
    High,
    VeryHigh,
    UltraHigh,
}
impl Size {
    pub const fn as_str(&self) -> &str {
        match self {
            Size::UltraLow => "Ultra Low",
            Size::VeryLow => "Very Low",
            Size::Low => "Low",
            Size::Medium => "Medium",
            Size::High => "High",
            Size::VeryHigh => "Very High",
            Size::UltraHigh => "Ultra High",
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ratio {
    #[default]
    Standard,
    Square,
    Classic,
}

impl Ratio {
    pub const fn as_str(&self) -> &str {
        match self {
            Ratio::Standard => "Standard (16:9)",
            Ratio::Square => "Square (1:1)",
            Ratio::Classic => "Classic (4:3)",
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub size: Size,
    pub ratio: Ratio,
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
            Ratio::Standard => standard_resolution(self.size),
            Ratio::Square => square_resolution(self.size),
            Ratio::Classic => classic_resolution(self.size),
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
