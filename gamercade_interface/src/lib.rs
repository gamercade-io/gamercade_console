mod output;
pub use output::*;

mod network_session;
pub use network_session::*;

pub mod security;

pub mod filter;

pub const USERNAME_LENGTH_MIN: usize = 1;
pub const USERNAME_LENGTH_MAX: usize = 32;

pub const SESSION_METADATA_KEY: &str = "gc-session";

#[derive(Debug)]
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
}

impl Into<Vec<u8>> for Session {
    fn into(self) -> Vec<u8> {
        Vec::from(self.0)
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
