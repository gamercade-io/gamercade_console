mod output;
pub use output::*;

mod network_session;
pub use network_session::*;

pub mod security;

pub mod filter;

pub const USERNAME_LENGTH_MIN: usize = 1;
pub const USERNAME_LENGTH_MAX: usize = 31;
pub const EMAIL_MAX_LENGTH: usize = 255;

pub const GAME_TITLE_MAX_LENGTH: usize = 123;
pub const GAME_SHORT_DESCRIPTION_MAX_LENGTH: usize = 255;
pub const GAME_LONG_DESCRIPTION_MAX_LENGTH: usize = 2047;
pub const RELEASE_NAME_MAX_LENGTH: usize = 123;

pub const AUTHOR_TITLE_MAX_LENGTH: usize = 31;
pub const REVIEW_COMMENTS_MAX_LENGTH: usize = 1027;

pub const PERMISSION_LEVEL_EDITOR: i32 = 10;
pub const PERMISSION_LEVEL_OWNER: i32 = 0;

pub const SESSION_METADATA_KEY: &str = "gc-session-bin";
pub const URL_RADIX: usize = 16;

pub const IMAGE_MAX_SIZE_BYTES: usize = 1024 * 1024 * 3; // 3mb
pub const IMAGE_MAX_WIDTH: usize = 300;
pub const IMAGE_MAX_HEIGHT: usize = 300;

// Checksum Calculator
pub const CRC: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_XZ);

#[derive(Debug, Clone)]
pub struct Session([u8; 16]);

impl Session {
    pub fn get_metadata_key() -> &'static str {
        SESSION_METADATA_KEY
    }

    pub fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from_iter(self.0)
    }
}

impl From<Vec<u8>> for Session {
    fn from(value: Vec<u8>) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl TryFrom<&[u8]> for Session {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(
            value.try_into().map_err(|_| "Error converting session.")?,
        ))
    }
}

impl From<u128> for Session {
    fn from(value: u128) -> Self {
        Self(value.to_le_bytes())
    }
}
