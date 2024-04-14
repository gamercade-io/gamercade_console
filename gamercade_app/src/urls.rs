pub const SERVICE_IP_GRPC: &str = "http://127.0.0.1:50051";
pub const SERVICE_IP_HTTP: &str = "http://127.0.0.1:3000";

pub fn download_release_url(game_id: u64, release_id: u64) -> String {
    format!("{SERVICE_IP_HTTP}/games/{game_id}/releases/{release_id}")
}
