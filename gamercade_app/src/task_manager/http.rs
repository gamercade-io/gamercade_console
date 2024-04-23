use std::sync::Arc;

use gamercade_interface::{Session, SESSION_METADATA_KEY};

use nohash_hasher::IntMap;
use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc::Sender, Mutex},
};

use crate::{
    game_rom_path,
    task_manager::{DownloadRomComplete, TaskNotification},
    urls::{self, WithSession},
    GAME_DIR,
};

use super::{TaskManager, TaskRequest};

pub type HttpManager = TaskManager<HttpManagerState, HttpRequest>;

#[derive(Default)]
pub struct HttpManagerState {
    pub rom_downloads: IntMap<i64, ActiveDownload>,
    pub image_downloads: IntMap<i64, ActiveDownload>,
}

pub struct ActiveDownload {
    id: i64,
    download_status: DownloadStatus,
}

pub enum DownloadStatus {
    InProgress {
        bytes_downloaded: usize,
        total_bytes: usize,
    },
    Done(Vec<u8>),
}

#[derive(Debug)]
pub enum HttpRequest {
    DownloadRom(WithSession<DownloadRom>),
    UploadRom(WithSession<UploadRom>),
}

#[derive(Debug)]
pub enum HttpResponse {
    DownloadComplete(DownloadRomComplete),
    Upload(Result<(), String>),
}

#[derive(Debug)]
pub struct DownloadRom {
    pub game_id: i64,
}

#[derive(Debug)]
pub struct UploadRom {
    pub game_id: i64,
    pub bytes: Vec<u8>,
}

impl TaskRequest<HttpManagerState> for HttpRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &Arc<Mutex<HttpManagerState>>,
    ) {
        match self {
            HttpRequest::DownloadRom(request) => download_file(sender.clone(), request),
            HttpRequest::UploadRom(request) => {
                let WithSession {
                    data: request,
                    session,
                } = request;
                let session = u128::from_ne_bytes(*session.bytes());
                match reqwest::Client::new()
                    .post(urls::game_rom_url(request.game_id))
                    .header(SESSION_METADATA_KEY, format!("{session:x}"))
                    .body(request.bytes)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Err(_) = response.error_for_status_ref() {
                            sender
                                .send(TaskNotification::HttpResponse(HttpResponse::Upload(Err(
                                    response.text().await.unwrap_or_default(),
                                ))))
                                .await
                                .unwrap()
                        } else {
                            sender
                                .send(TaskNotification::HttpResponse(HttpResponse::Upload(Ok(()))))
                                .await
                                .unwrap()
                        }
                    }
                    Err(err) => sender
                        .send(TaskNotification::HttpResponse(HttpResponse::Upload(Err(
                            err.to_string(),
                        ))))
                        .await
                        .unwrap(),
                }
            }
        }
    }
}

// TODO: May want to keep the join handle around
fn download_file(sender: Sender<TaskNotification>, request: WithSession<DownloadRom>) {
    tokio::spawn(async move {
        let WithSession {
            session,
            data: request,
        } = request;
        let session = format!("{:x}", u128::from_ne_bytes(*session.bytes()));

        match reqwest::Client::new()
            .get(urls::game_rom_url(request.game_id))
            .header(SESSION_METADATA_KEY, session)
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
                            let _ = tokio::fs::create_dir_all(GAME_DIR).await;

                            // Create and write to the file
                            match tokio::fs::OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(game_rom_path(request.game_id))
                                .await
                            {
                                Ok(mut file) => {
                                    if let Err(e) = file.write_all(&buffer).await {
                                        println!("Error writing file: {e}");
                                        return;
                                    }

                                    // Notify the main thread that the download is done
                                    sender
                                        .send(TaskNotification::HttpResponse(
                                            HttpResponse::DownloadComplete(DownloadRomComplete {
                                                game_id: request.game_id,
                                                data: buffer,
                                            }),
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

impl HttpManager {
    pub fn try_download_rom(&mut self, game_id: i64, session: &Session) {
        self.sender
            .try_send(HttpRequest::DownloadRom(WithSession {
                session: session.clone(),
                data: DownloadRom { game_id },
            }))
            .unwrap();
    }

    pub fn try_upload_rom(&mut self, request: UploadRom, session: &Session) {
        self.sender
            .try_send(HttpRequest::UploadRom(WithSession {
                session: session.clone(),
                data: request,
            }))
            .unwrap()
    }
}
