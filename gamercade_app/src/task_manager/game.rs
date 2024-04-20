use std::sync::Arc;

use gamercade_interface::{
    game::{game_service_client::GameServiceClient, UpdateGameRequest},
    Session,
};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::{transport::Channel, Request};

use crate::urls::{WithSession, SERVICE_IP_GRPC};

use super::{TaskManager, TaskRequest};

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

impl TaskRequest<GameManagerState> for GameRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &Arc<Mutex<GameManagerState>>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_game_client).await;
        let client = lock.client.get_mut().unwrap();

        let result = match self {
            GameRequest::CreateGame(request) => {
                client.create_game(request.authorized_request()).await
            }
            GameRequest::UpdateGame(request) => {
                client.update_game(request.authorized_request()).await
            }
        };

        match result {
            Ok(game_info) => println!("Got game info: {game_info:?}"),
            Err(e) => println!("{e}"),
        }
    }
}
