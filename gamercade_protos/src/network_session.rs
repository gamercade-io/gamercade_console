use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NetworkSession {
    pub players: Box<[NetworkedClient]>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkedClient {
    pub count: usize,
    pub kind: NetworkedPlayerType,
}

#[derive(Serialize, Deserialize)]
pub enum NetworkedPlayerType {
    Local,
    Remote(SocketAddr),
}
