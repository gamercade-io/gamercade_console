use gamercade_interface::release::release_service_client::ReleaseServiceClient;
use hyper::Request;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::urls;

use super::{TaskManager, TaskRequest};

pub type ReleaseManager = TaskManager<ReleaseManagerState, ReleaseRequest>;

#[derive(Default)]
pub struct ReleaseManagerState {
    downloads: Vec<DownloadReleaseRequest>,
    client: OnceCell<ReleaseServiceClient<Channel>>,
}

#[derive(Debug)]
pub enum ReleaseRequest {
    DownloadRelease(DownloadReleaseRequest),
}

#[derive(Debug)]
pub struct DownloadReleaseRequest {
    game_id: u64,
    release_id: u64,
}

impl TaskRequest<ReleaseManagerState> for ReleaseRequest {
    async fn handle_request(
        self,
        sender: &tokio::sync::mpsc::Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<ReleaseManagerState>,
    ) {
        let mut lock = state.lock().await;

        match self {
            ReleaseRequest::DownloadRelease(request) => {
                let requst = Request::builder().uri(urls::download_release_url(
                    request.game_id,
                    request.release_id,
                ));
                // TODO: This
                todo!()
            }
        }
    }
}

impl ReleaseManager {
    pub fn try_download(&mut self, game_id: u64, release_id: u64) {
        self.sender
            .try_send(ReleaseRequest::DownloadRelease(DownloadReleaseRequest {
                game_id,
                release_id,
            }))
            .unwrap();
    }
}
