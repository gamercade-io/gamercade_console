use std::str::FromStr;

use gamercade_interface::{Session, SESSION_METADATA_KEY};
use tonic::{metadata::MetadataValue, Request};

pub const SERVICE_IP_GRPC: &str = "http://127.0.0.1:50051";
pub const SERVICE_IP_HTTP: &str = "http://127.0.0.1:3000";

pub fn download_release_url(game_id: u64, release_id: u64) -> String {
    format!("{SERVICE_IP_HTTP}/games/{game_id}/releases/{release_id}")
}

#[derive(Debug)]
pub struct WithSession<T> {
    pub session: Session,
    pub data: T,
}

impl<T> WithSession<T> {
    pub fn new(session: &Session, data: T) -> Self {
        Self {
            session: session.clone(),
            data,
        }
    }

    pub fn authorized_request(self) -> Request<T> {
        let mut request = Request::new(self.data);
        request.metadata_mut().insert_bin(
            SESSION_METADATA_KEY,
            MetadataValue::from_bytes(self.session.bytes()),
        );
        request
    }
}