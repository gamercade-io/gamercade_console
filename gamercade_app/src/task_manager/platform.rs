use std::sync::Arc;

use gamercade_interface::{
    common::Empty,
    game::MultipleGamesInfoResponse,
    platform::{
        platform_service_client::PlatformServiceClient, EditableGamesResponse, FrontPageRequest,
        FrontPageResponse, GameSearchRequest, VotedGamesResponse,
    },
    Session,
};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::transport::Channel;

use crate::urls::{WithSession, SERVICE_IP_GRPC};

use super::{TaskManager, TaskNotification, TaskRequest};

pub type PlatformManager = TaskManager<PlatformManagerState, PlatformRequest>;

async fn init_platform_client() -> PlatformServiceClient<Channel> {
    PlatformServiceClient::connect(SERVICE_IP_GRPC)
        .await
        .unwrap()
}

#[derive(Default)]
pub struct PlatformManagerState {
    client: OnceCell<PlatformServiceClient<Channel>>,
}

#[derive(Debug)]
pub enum PlatformRequest {
    FrontPage(FrontPageRequest),
    Search(GameSearchRequest),
    EditableGames(Session),
    VotedGames(Session),
}

#[derive(Debug)]
pub enum PlatformResponse {
    FrontPage(FrontPageResponse),
    EditableGames(EditableGamesResponse),
    VotedGames(VotedGamesResponse),
    Search(MultipleGamesInfoResponse),
}

impl TaskRequest<PlatformManagerState> for PlatformRequest {
    async fn handle_request(
        self,
        sender: &Sender<super::TaskNotification>,
        state: &Arc<Mutex<PlatformManagerState>>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_platform_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            PlatformRequest::FrontPage(request) => match client.front_page(request).await {
                Ok(response) => sender
                    .try_send(TaskNotification::PlatformResponse(
                        PlatformResponse::FrontPage(response.into_inner()),
                    ))
                    .unwrap(),
                Err(err) => println!("front page response err: {err}"),
            },
            PlatformRequest::Search(request) => match client.game_search(request).await {
                Ok(response) => sender
                    .send(TaskNotification::PlatformResponse(
                        PlatformResponse::Search(response.into_inner()),
                    ))
                    .await
                    .unwrap(),
                Err(err) => println!("search response err: {err}"),
            },
            PlatformRequest::EditableGames(session) => match client
                .get_editable_games(WithSession::new(&session, Empty {}).authorized_request())
                .await
            {
                Ok(response) => sender
                    .send(TaskNotification::PlatformResponse(
                        PlatformResponse::EditableGames(response.into_inner()),
                    ))
                    .await
                    .unwrap(),
                Err(err) => println!("editable games response err: {err}"),
            },
            PlatformRequest::VotedGames(session) => match client
                .get_voted_games(WithSession::new(&session, Empty {}).authorized_request())
                .await
            {
                Ok(response) => sender
                    .send(TaskNotification::PlatformResponse(
                        PlatformResponse::VotedGames(response.into_inner()),
                    ))
                    .await
                    .unwrap(),
                Err(err) => println!("voted games response err: {err}"),
            },
        }
    }
}
