use serde::{self, Deserialize, Deserializer, Serialize};

use crate::{BYTES_PER_PIXEL, PALETTE_COLORS};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorIndex(pub u8);

impl TryFrom<i32> for ColorIndex {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value < PALETTE_COLORS as i32 {
            Ok(Self(value as u8))
        } else {
            Err("invalid color index")
        }
    }

    type Error = &'static str;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = if deserializer.is_human_readable() {
            let hex: String = Deserialize::deserialize(deserializer)?;
            u32::from_str_radix(&hex, 16).map_err(serde::de::Error::custom)?
        } else {
            Deserialize::deserialize(deserializer)?
        };
        Ok(Color::from_hex(hex))
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{:x}", self.to_hex()))
        } else {
            serializer.serialize_u32(self.to_hex())
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        let r = (self.r as u32) << 24;
        let g = (self.g as u32) << 16;
        let b = (self.b as u32) << 8;
        let a = self.a as u32;

        r | g | b | a
    }

    pub fn into_pixel_data(&self) -> [u8; BYTES_PER_PIXEL] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}
