use std::sync::Arc;

use gamercade_interface::game::{
    game_service_client::GameServiceClient, GameInfoBasic, UpdateGameRequest,
};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::{transport::Channel, Response};

use crate::urls::{WithSession, SERVICE_IP_GRPC};

use super::{TaskManager, TaskNotification, TaskRequest};

pub type GameManager = TaskManager<GameManagerState, GameRequest>;

async fn init_game_client() -> GameServiceClient<Channel> {
    GameServiceClient::connect(SERVICE_IP_GRPC).await.unwrap()
}

#[derive(Default)]
pub struct GameManagerState {
    client: OnceCell<GameServiceClient<Channel>>,
}

#[derive(Debug)]
pub enum GameRequest {
    CreateGame(WithSession<UpdateGameRequest>),
    UpdateGame(WithSession<UpdateGameRequest>),
}

#[derive(Debug)]
pub enum GameResponse {
    CreateGame(Result<GameInfoBasic, String>),
    UpdateGame(Result<GameInfoBasic, String>),
}

impl TaskRequest<GameManagerState> for GameRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &Arc<Mutex<GameManagerState>>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_game_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            GameRequest::CreateGame(request) => {
                let response = client.create_game(request.authorized_request()).await;
                sender
                    .send(TaskNotification::GameResponse(GameResponse::CreateGame(
                        response
                            .map(Response::into_inner)
                            .map_err(|e| e.to_string()),
                    )))
                    .await
                    .unwrap()
            }
            GameRequest::UpdateGame(request) => {
                let response = client.update_game(request.authorized_request()).await;
                sender
                    .send(TaskNotification::GameResponse(GameResponse::UpdateGame(
                        response
                            .map(Response::into_inner)
                            .map_err(|e| e.to_string()),
                    )))
                    .await
                    .unwrap()
            }
        };
    }
}
