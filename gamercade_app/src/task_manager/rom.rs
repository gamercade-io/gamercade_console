use gamercade_interface::{Session, SESSION_METADATA_KEY};

use tokio::{io::AsyncWriteExt, sync::mpsc::Sender};

use crate::{
    task_manager::{DownloadRomComplete, TaskNotification},
    urls::{self, WithSession},
};

use super::{TaskManager, TaskRequest};

pub type RomManager = TaskManager<RomManagerState, RomRequest>;

#[derive(Default)]
pub struct RomManagerState;

#[derive(Debug)]
pub enum RomRequest {
    DownloadRom(WithSession<DownloadRom>),
    UploadRom(WithSession<UploadRom>),
}

#[derive(Debug)]
pub struct DownloadRom {
    pub game_id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct UploadRom {
    pub game_id: i64,
    pub bytes: Vec<u8>,
}

fn game_dir(game_id: i64) -> String {
    format!("./roms/{game_id:x}")
}

fn game_rom_path(game_id: i64, name: &str) -> String {
    format!("./roms/{game_id:x}/{name}.gcrom")
}

impl TaskRequest<RomManagerState> for RomRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<RomManagerState>,
    ) {
        match self {
            RomRequest::DownloadRom(request) => download_file(sender.clone(), request),
            RomRequest::UploadRom(request) => {
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
                        let status = response.status();
                        let body = response.text().await;
                        println!("Upload rom response ({status}): {body:?}");
                    }
                    Err(err) => println!("Upload rom Error: {err}"),
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
                            let _ = tokio::fs::create_dir_all(game_dir(request.game_id)).await;

                            // Create and write to the file
                            match tokio::fs::OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(game_rom_path(request.game_id, &request.name))
                                .await
                            {
                                Ok(mut file) => {
                                    if let Err(e) = file.write_all(&buffer).await {
                                        println!("Error writing file: {e}");
                                        return;
                                    }

                                    // Notify the main thread that the download is done
                                    sender
                                        .send(TaskNotification::DownloadRomComplete(
                                            DownloadRomComplete {
                                                game_id: request.game_id,
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

impl RomManager {
    pub fn try_download_rom(&mut self, game_id: i64, rom_name: &str, session: &Session) {
        self.sender
            .try_send(RomRequest::DownloadRom(WithSession {
                session: session.clone(),
                data: DownloadRom {
                    game_id,
                    name: rom_name.to_string(),
                },
            }))
            .unwrap();
    }

    pub fn try_upload_rom(&mut self, request: UploadRom, session: &Session) {
        self.sender
            .try_send(RomRequest::UploadRom(WithSession {
                session: session.clone(),
                data: request,
            }))
            .unwrap()
    }
}
