use serde::{Deserialize, Serialize};
use std::ops;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Resolution(ResolutionLevels);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolutionLevels {
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

    pub const fn new(resolution: ResolutionLevels) -> Self {
        Resolution(resolution)
    }

    pub const fn width(&self) -> i32 {
        match self.0 {
            ResolutionLevels::UltraLow => Self::ULTRALOW.0,
            ResolutionLevels::VeryLow => Self::VERYLOW.0,
            ResolutionLevels::Low => Self::LOW.0,
            ResolutionLevels::Medium => Self::MEDIUM.0,
            ResolutionLevels::High => Self::HIGH.0,
            ResolutionLevels::VeryHigh => Self::VERYHIGH.0,
            ResolutionLevels::UltraHigh => Self::ULTRAHIGH.0,
        }
    }

    pub const fn height(&self) -> i32 {
        match self.0 {
            ResolutionLevels::UltraLow => Self::ULTRALOW.1,
            ResolutionLevels::VeryLow => Self::VERYLOW.1,
            ResolutionLevels::Low => Self::LOW.1,
            ResolutionLevels::Medium => Self::MEDIUM.1,
            ResolutionLevels::High => Self::HIGH.1,
            ResolutionLevels::VeryHigh => Self::VERYHIGH.1,
            ResolutionLevels::UltraHigh => Self::ULTRAHIGH.1,
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
        Self(ResolutionLevels::VeryLow)
    }
}

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct XCord(usize);

#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct YCord(usize);

impl XCord {
    pub fn raw_value(&self) -> usize {
        self.0
    }
}

impl YCord {
    pub fn raw_value(&self) -> usize {
        self.0
    }
}

// Derive Multiplication
impl_op_ex!(*|a: &XCord, b: &XCord| -> XCord { XCord(a.0 * b.0) });
impl_op_ex!(*|a: &XCord, b: &YCord| -> XCord { XCord(a.0 * b.0) });
impl_op_ex!(*|a: &YCord, b: &YCord| -> YCord { YCord(a.0 * b.0) });
impl_op_ex!(*|a: &YCord, b: &XCord| -> YCord { YCord(a.0 * b.0) });
impl_op_ex!(*|a: &XCord, b: &usize| -> XCord { XCord(a.0 * b) });
impl_op_ex!(*|a: &YCord, b: &usize| -> YCord { YCord(a.0 * b) });

// Derive Addition
impl_op_ex!(+ |a: &XCord, b: &XCord| -> XCord {XCord(a.0 + b.0)});
impl_op_ex!(+ |a: &XCord, b: &YCord| -> XCord {XCord(a.0 + b.0)});
impl_op_ex!(+ |a: &YCord, b: &YCord| -> YCord {YCord(a.0 + b.0)});
impl_op_ex!(+ |a: &YCord, b: &XCord| -> YCord {YCord(a.0 + b.0)});
impl_op_ex!(+ |a: &XCord, b: &usize| -> XCord {XCord(a.0 + b)});
impl_op_ex!(+ |a: &YCord, b: &usize| -> YCord {YCord(a.0 + b)});

// Derive Subtraction
impl_op_ex!(-|a: &XCord, b: &XCord| -> XCord { XCord(a.0.max(b.0) - a.0.min(b.0)) });
impl_op_ex!(-|a: &XCord, b: &YCord| -> XCord { XCord(a.0.max(b.0) - a.0.min(b.0)) });
impl_op_ex!(-|a: &YCord, b: &YCord| -> YCord { YCord(a.0.max(b.0) - a.0.min(b.0)) });
impl_op_ex!(-|a: &YCord, b: &XCord| -> YCord { YCord(a.0.max(b.0) - a.0.min(b.0)) });
impl_op_ex!(-|a: &XCord, b: &usize| -> XCord { XCord(a.0.max(*b) - a.0.min(*b)) });
impl_op_ex!(-|a: &YCord, b: &usize| -> YCord { YCord(a.0.max(*b) - a.0.min(*b)) });
