use std::{fmt::Display, ops::Index, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::BYTES_PER_PIXEL;

use super::{Color, ColorIndex, PALETTE_COLORS};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaletteIndex(pub u8);

impl FromStr for PaletteIndex {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Self(num))
        } else {
            Err("couldn't parse PaletteIndex from str")
        }
    }
}

impl Display for PaletteIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    pub colors: [Color; PALETTE_COLORS],
}

impl Index<ColorIndex> for Palette {
    type Output = Color;

    fn index(&self, index: ColorIndex) -> &Self::Output {
        &self.colors[index.0]
    }
}

impl Palette {
    /// Gets the raw color, ignoring any kind of transparency
    pub fn color(&self, color_index: ColorIndex) -> Color {
        self[color_index]
    }

    // TODO: could be optimized using unint stuff?
    pub fn as_pixel_colors(&self) -> [[u8; BYTES_PER_PIXEL]; PALETTE_COLORS] {
        self.colors.map(|color| color.into_pixel_data())
    }

    pub fn default_palette_collection() -> Vec<Palette> {
        vec![
            Self::bubblegum16(),
            Self::islandjoy16(),
            Self::pico8(),
            Self::sweetie16(),
            Self::na16(),
            Self::steamlords(),
            Self::endesga16(),
            Self::vanillamilkshake(),
        ]
    }

    pub fn bubblegum16() -> Self {
        Self {
            colors: [
                Color::new(0x16, 0x17, 0x1a),
                Color::new(0x7f, 0x06, 0x22),
                Color::new(0xd6, 0x24, 0x11),
                Color::new(0xff, 0x84, 0x26),
                Color::new(0xff, 0xd1, 0x00),
                Color::new(0xfa, 0xfd, 0xff),
                Color::new(0xff, 0x80, 0xa4),
                Color::new(0xff, 0x26, 0x74),
                Color::new(0x94, 0x21, 0x6a),
                Color::new(0x43, 0x00, 0x67),
                Color::new(0x23, 0x49, 0x75),
                Color::new(0x68, 0xae, 0xd4),
                Color::new(0xbf, 0xff, 0x3c),
                Color::new(0x10, 0xd2, 0x75),
                Color::new(0x00, 0x78, 0x99),
                Color::new(0x00, 0x28, 0x59),
            ],
        }
    }

    pub fn islandjoy16() -> Self {
        Self {
            colors: [
                Color::new(0xff, 0xff, 0xff),
                Color::new(0x6d, 0xf7, 0xc1),
                Color::new(0x11, 0xad, 0xc1),
                Color::new(0x60, 0x6c, 0x81),
                Color::new(0x39, 0x34, 0x57),
                Color::new(0x1e, 0x88, 0x75),
                Color::new(0x5b, 0xb3, 0x61),
                Color::new(0xa1, 0xe5, 0x5a),
                Color::new(0xf7, 0xe4, 0x76),
                Color::new(0xf9, 0x92, 0x52),
                Color::new(0xcb, 0x4d, 0x68),
                Color::new(0x6a, 0x37, 0x71),
                Color::new(0xc9, 0x24, 0x64),
                Color::new(0xf4, 0x8c, 0xb6),
                Color::new(0xf7, 0xb6, 0x9e),
                Color::new(0x9b, 0x9c, 0x82),
            ],
        }
    }

    pub fn pico8() -> Self {
        Self {
            colors: [
                Color::new(0x00, 0x00, 0x00),
                Color::new(0x1D, 0x2B, 0x53),
                Color::new(0x7E, 0x25, 0x53),
                Color::new(0x00, 0x87, 0x51),
                Color::new(0xAB, 0x52, 0x36),
                Color::new(0x5F, 0x57, 0x4F),
                Color::new(0xC2, 0xC3, 0xC7),
                Color::new(0xFF, 0xF1, 0xE8),
                Color::new(0xFF, 0x00, 0x4D),
                Color::new(0xFF, 0xA3, 0x00),
                Color::new(0xFF, 0xEC, 0x27),
                Color::new(0x00, 0xE4, 0x36),
                Color::new(0x29, 0xAD, 0xFF),
                Color::new(0x83, 0x76, 0x9C),
                Color::new(0xFF, 0x77, 0xA8),
                Color::new(0xFF, 0xCC, 0xAA),
            ],
        }
    }

    pub fn sweetie16() -> Self {
        Self {
            colors: [
                Color::new(0x1a, 0x1c, 0x2c),
                Color::new(0x5d, 0x27, 0x5d),
                Color::new(0xb1, 0x3e, 0x53),
                Color::new(0xef, 0x7d, 0x57),
                Color::new(0xff, 0xcd, 0x75),
                Color::new(0xa7, 0xf0, 0x70),
                Color::new(0x38, 0xb7, 0x64),
                Color::new(0x25, 0x71, 0x79),
                Color::new(0x29, 0x36, 0x6f),
                Color::new(0x3b, 0x5d, 0xc9),
                Color::new(0x41, 0xa6, 0xf6),
                Color::new(0x73, 0xef, 0xf7),
                Color::new(0xf4, 0xf4, 0xf4),
                Color::new(0x94, 0xb0, 0xc2),
                Color::new(0x56, 0x6c, 0x86),
                Color::new(0x33, 0x3c, 0x57),
            ],
        }
    }

    pub fn na16() -> Self {
        Self {
            colors: [
                Color::new(0x8c, 0x8f, 0xae),
                Color::new(0x58, 0x45, 0x63),
                Color::new(0x3e, 0x21, 0x37),
                Color::new(0x9a, 0x63, 0x48),
                Color::new(0xd7, 0x9b, 0x7d),
                Color::new(0xf5, 0xed, 0xba),
                Color::new(0xc0, 0xc7, 0x41),
                Color::new(0x64, 0x7d, 0x34),
                Color::new(0xe4, 0x94, 0x3a),
                Color::new(0x9d, 0x30, 0x3b),
                Color::new(0xd2, 0x64, 0x71),
                Color::new(0x70, 0x37, 0x7f),
                Color::new(0x7e, 0xc4, 0xc1),
                Color::new(0x34, 0x85, 0x9d),
                Color::new(0x17, 0x43, 0x4b),
                Color::new(0x1f, 0x0e, 0x1c),
            ],
        }
    }

    pub fn steamlords() -> Self {
        Self {
            colors: [
                Color::new(0x21, 0x3b, 0x25),
                Color::new(0x3a, 0x60, 0x4a),
                Color::new(0x4f, 0x77, 0x54),
                Color::new(0xa1, 0x9f, 0x7c),
                Color::new(0x77, 0x74, 0x4f),
                Color::new(0x77, 0x5c, 0x4f),
                Color::new(0x60, 0x3b, 0x3a),
                Color::new(0x3b, 0x21, 0x37),
                Color::new(0x17, 0x0e, 0x19),
                Color::new(0x2f, 0x21, 0x3b),
                Color::new(0x43, 0x3a, 0x60),
                Color::new(0x4f, 0x52, 0x77),
                Color::new(0x65, 0x73, 0x8c),
                Color::new(0x7c, 0x94, 0xa1),
                Color::new(0xa0, 0xb9, 0xba),
                Color::new(0xc0, 0xd1, 0xcc),
            ],
        }
    }

    pub fn endesga16() -> Self {
        Self {
            colors: [
                Color::new(0xe4, 0xa6, 0x72),
                Color::new(0xb8, 0x6f, 0x50),
                Color::new(0x74, 0x3f, 0x39),
                Color::new(0x3f, 0x28, 0x32),
                Color::new(0x9e, 0x28, 0x35),
                Color::new(0xe5, 0x3b, 0x44),
                Color::new(0xfb, 0x92, 0x2b),
                Color::new(0xff, 0xe7, 0x62),
                Color::new(0x63, 0xc6, 0x4d),
                Color::new(0x32, 0x73, 0x45),
                Color::new(0x19, 0x3d, 0x3f),
                Color::new(0x4f, 0x67, 0x81),
                Color::new(0xaf, 0xbf, 0xd2),
                Color::new(0xff, 0xff, 0xff),
                Color::new(0x2c, 0xe8, 0xf4),
                Color::new(0x04, 0x84, 0xd1),
            ],
        }
    }

    pub fn vanillamilkshake() -> Self {
        Self {
            colors: [
                Color::new(0x28, 0x28, 0x2e),
                Color::new(0x6c, 0x56, 0x71),
                Color::new(0xd9, 0xc8, 0xbf),
                Color::new(0xf9, 0x82, 0x84),
                Color::new(0xb0, 0xa9, 0xe4),
                Color::new(0xac, 0xcc, 0xe4),
                Color::new(0xb3, 0xe3, 0xda),
                Color::new(0xfe, 0xaa, 0xe4),
                Color::new(0x87, 0xa8, 0x89),
                Color::new(0xb0, 0xeb, 0x93),
                Color::new(0xe9, 0xf5, 0x9d),
                Color::new(0xff, 0xe6, 0xc6),
                Color::new(0xde, 0xa3, 0x8b),
                Color::new(0xff, 0xc3, 0x84),
                Color::new(0xff, 0xf7, 0xa0),
                Color::new(0xff, 0xf7, 0xe4),
            ],
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            colors: [Color::default(); PALETTE_COLORS],
        }
    }
}
