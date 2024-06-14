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

    pub fn to_hex_string(&self) -> String {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn from_hex_string(hex: &str) -> Result<Self, &'static str> {
        match (hex.starts_with('#'), hex.len()) {
            (true, 7) => {
                let hex_value =
                    u32::from_str_radix(&hex[1..], 16).map_err(|_| "Invalid hex string")?;
                let r = ((hex_value >> 16) & 0xff) as u8;
                let g = ((hex_value >> 8) & 0xff) as u8;
                let b = (hex_value & 0xff) as u8;
                return Ok(Self { r, g, b, a: 255 });
            }
            (false, 6) => {
                let hex_value = u32::from_str_radix(&hex, 16).map_err(|_| "Invalid hex string")?;
                let r = ((hex_value >> 16) & 0xff) as u8;
                let g = ((hex_value >> 8) & 0xff) as u8;
                let b = (hex_value & 0xff) as u8;
                return Ok(Self { r, g, b, a: 255 });
            }
            _ => {
                return Err("Invalid hex string");
            }
        }
    }

    pub fn update_from_hex_string(&mut self, hex: &str) -> Result<(), &'static str> {
        *self = Self::from_hex_string(hex)?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex_string() {
        let color = Color::from_hex_string("#FF0000").unwrap();
        assert_eq!(color, Color::new(255, 0, 0, 255));

        let color = Color::from_hex_string("#000000").unwrap();
        assert_eq!(color, Color::new(0, 0, 0, 255));

        let color = Color::from_hex_string("FF0000").unwrap();
        assert_eq!(color, Color::new(255, 0, 0, 255));

        let color = Color::from_hex_string("ffffff").unwrap();
        assert_eq!(color, Color::new(255, 255, 255, 255));

        assert!(Color::from_hex_string("#FF000").is_err());
        assert!(Color::from_hex_string("#FF00000").is_err());
        assert!(Color::from_hex_string("#FF00000FF").is_err());
        assert!(Color::from_hex_string("FF00000FF").is_err());
        assert!(Color::from_hex_string("asd").is_err());
        assert!(Color::from_hex_string("pppppppp").is_err());
        assert!(Color::from_hex_string("#pppppp").is_err());
        assert!(Color::from_hex_string("#FFFFFFFF").is_err());
    }
}
