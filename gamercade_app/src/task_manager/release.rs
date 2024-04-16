use gamercade_interface::{
    release::{release_service_client::ReleaseServiceClient, CreateReleaseRequest},
    Session, SESSION_METADATA_KEY,
};

use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc::Sender, OnceCell},
};
use tonic::transport::Channel;

use crate::{
    task_manager::{DownloadReleaseComplete, TaskNotification},
    urls::{self, WithSession, SERVICE_IP_GRPC},
};

use super::{TaskManager, TaskRequest};

pub type ReleaseManager = TaskManager<ReleaseManagerState, ReleaseRequest>;

#[derive(Default)]
pub struct ReleaseManagerState {
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

fn game_dir(game_id: i64) -> String {
    format!("./roms/{game_id:x}")
}

fn game_release_name(game_id: i64, release_id: i64) -> String {
    format!("./roms/{game_id:x}/{release_id:x}.gcrom")
}

impl TaskRequest<ReleaseManagerState> for ReleaseRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<ReleaseManagerState>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_release_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            ReleaseRequest::DownloadRelease(request) => download_file(sender.clone(), request),
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

// TODO: May want to keep the join handle around
fn download_file(sender: Sender<TaskNotification>, request: DownloadReleaseRequest) {
    tokio::spawn(async move {
        match reqwest::Client::new()
            .get(urls::game_release_url(request.game_id, request.release_id))
            .send()
            .await
        {
            Ok(mut response) => {
                println!("download response: {response:?}");

                if let Err(e) = response.error_for_status_ref() {
                    println!("Error response: {e}");
                    println!("{:?}", response.text().await);
                    return;
                }

                let mut buffer = Vec::new();
                loop {
                    match response.chunk().await {
                        Ok(Some(bytes)) => {
                            buffer.extend_from_slice(&bytes);
                        }
                        Ok(None) => {
                            // Create the directory
                            let _ = tokio::fs::create_dir_all(game_dir(request.game_id)).await;

                            // Create and write to the file
                            match tokio::fs::OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(game_release_name(request.game_id, request.release_id))
                                .await
                            {
                                Ok(mut file) => {
                                    if let Err(e) = file.write_all(&buffer).await {
                                        println!("Error writing file: {e}");
                                        return;
                                    }

                                    // Notify the main thread that the download is done
                                    sender
                                        .send(TaskNotification::DownloadReleaseComplete(
                                            DownloadReleaseComplete {
                                                game_id: request.game_id,
                                                release_id: request.release_id,
                                                data: buffer,
                                            },
                                        ))
                                        .await
                                        .unwrap();
                                }
                                Err(e) => println!("Error writing file: {e}"), //TOOD: Could send an error that the download failed
                            };

                            break;
                        }
                        Err(e) => {
                            println!("Download error: {e}");
                            break;
                        }
                    }
                }
            }
            Err(err) => println!("Download release Error: {err}"),
        }
    });
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
