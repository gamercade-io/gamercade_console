use gamercade_interface::{
    release::{release_service_client::ReleaseServiceClient, CreateReleaseRequest},
    Session, SESSION_METADATA_KEY,
};

use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::urls::{self, WithSession, SERVICE_IP_GRPC};

use super::{TaskManager, TaskRequest};

pub type ReleaseManager = TaskManager<ReleaseManagerState, ReleaseRequest>;

#[derive(Default)]
pub struct ReleaseManagerState {
    downloads: Vec<DownloadReleaseRequest>,
    client: OnceCell<ReleaseServiceClient<Channel>>,
}

async fn init_release_client() -> ReleaseServiceClient<Channel> {
    ReleaseServiceClient::connect(SERVICE_IP_GRPC)
        .await
        .unwrap()
}

#[derive(Debug)]
pub enum ReleaseRequest {
    CreateRelease(WithSession<CreateReleaseRequest>),
    DownloadRelease(DownloadReleaseRequest),
    UploadRelease(WithSession<UploadReleaseRequest>),
}

#[derive(Debug)]
pub struct DownloadReleaseRequest {
    pub game_id: i64,
    pub release_id: i64,
}

#[derive(Debug)]
pub struct UploadReleaseRequest {
    pub game_id: i64,
    pub release_id: i64,
    pub bytes: Vec<u8>,
}

impl TaskRequest<ReleaseManagerState> for ReleaseRequest {
    async fn handle_request(
        self,
        _sender: &tokio::sync::mpsc::Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<ReleaseManagerState>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_release_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            ReleaseRequest::DownloadRelease(request) => {
                // TODO: This
                println!("TODO: Download a release");
            }
            ReleaseRequest::CreateRelease(request) => {
                match client
                    .create_new_release(request.authorized_request())
                    .await
                {
                    Ok(response) => println!("Release created: {response:?}"),
                    Err(e) => {
                        println!("Error creating release: {e}")
                    }
                }
            }
            ReleaseRequest::UploadRelease(request) => {
                let WithSession {
                    data: request,
                    session,
                } = request;
                let session = u128::from_ne_bytes(*session.bytes());
                match reqwest::Client::new()
                    .post(urls::game_release_url(request.game_id, request.release_id))
                    .header(SESSION_METADATA_KEY, format!("{session:x}"))
                    .body(request.bytes)
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        let body = response.text().await;
                        println!("Upload release response ({status}): {body:?}");
                    }
                    Err(err) => println!("Upload release Error: {err}"),
                }
            }
        }
    }
}

impl ReleaseManager {
    pub fn try_download(&mut self, game_id: i64, release_id: i64) {
        self.sender
            .try_send(ReleaseRequest::DownloadRelease(DownloadReleaseRequest {
                game_id,
                release_id,
            }))
            .unwrap();
    }

    pub fn try_create_release(&mut self, request: CreateReleaseRequest, session: &Session) {
        self.sender
            .try_send(ReleaseRequest::CreateRelease(WithSession {
                session: session.clone(),
                data: request,
            }))
            .unwrap()
    }

    pub fn try_upload_release(&mut self, request: UploadReleaseRequest, session: &Session) {
        self.sender
            .try_send(ReleaseRequest::UploadRelease(WithSession {
                session: session.clone(),
                data: request,
            }))
            .unwrap()
    }
}
