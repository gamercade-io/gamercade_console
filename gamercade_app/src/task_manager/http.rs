use std::{collections::hash_map::Entry, sync::Arc};

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
    pub id: i64,
    pub download_status: DownloadStatus,
}

pub enum DownloadStatus {
    Starting,
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
    DeleteRom(WithSession<i64>),
    DownloadImage(i64),
    UploadImage(WithSession<UploadImage>),
    DeleteImage(WithSession<i64>),
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

#[derive(Debug)]
pub struct UploadImage {
    pub game_id: i64,
    pub image: Vec<u8>,
}

impl TaskRequest<HttpManagerState> for HttpRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &Arc<Mutex<HttpManagerState>>,
    ) {
        match self {
            HttpRequest::DownloadRom(request) => {
                let mut lock = state.lock().await;

                if let Entry::Vacant(e) = lock.rom_downloads.entry(request.data.game_id) {
                    e.insert(ActiveDownload {
                        id: request.data.game_id,
                        download_status: DownloadStatus::Starting,
                    });
                    drop(lock);
                    download_file(sender.clone(), state.clone(), request);
                }
            }
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
            HttpRequest::DeleteRom(request) => {
                // TODO: Hit the platform service to delete the rom
                todo!()
            }
            HttpRequest::DownloadImage(request) => {
                // TODO: Hit the platform service and download the image file
                todo!()
            }
            HttpRequest::UploadImage(request) => {
                // TODO: Upload the image to the service
                todo!()
            }
            HttpRequest::DeleteImage(request) => {
                // TODO: Hit the platform service to delete the image
                todo!()
            }
        }
    }
}

// TODO: May want to keep the join handle around
fn download_file(
    sender: Sender<TaskNotification>,
    state: Arc<Mutex<HttpManagerState>>,
    request: WithSession<DownloadRom>,
) {
    tokio::spawn(async move {
        let WithSession {
            session,
            data: request,
        } = request;
        println!("Downloading file!");
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

                let total_bytes = response.content_length().unwrap_or_default() as usize;

                let mut buffer = Vec::new();
                loop {
                    match response.chunk().await {
                        Ok(Some(bytes)) => {
                            buffer.extend_from_slice(&bytes);

                            // Notify download progress
                            let mut lock = state.lock().await;
                            let download = lock.rom_downloads.entry(request.game_id).or_insert(
                                ActiveDownload {
                                    id: request.game_id,
                                    download_status: DownloadStatus::InProgress {
                                        bytes_downloaded: 0,
                                        total_bytes,
                                    },
                                },
                            );
                            if let DownloadStatus::InProgress {
                                bytes_downloaded, ..
                            } = &mut download.download_status
                            {
                                *bytes_downloaded += bytes.len();
                            }
                        }
                        Ok(None) => {
                            // Download is done

                            // Remove it from the active download list.
                            let mut lock = state.lock().await;
                            lock.rom_downloads.remove(&request.game_id);

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
                                    // TODO: Include checksum in output

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
