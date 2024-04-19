use gamercade_interface::{
    platform::{
        platform_service_client::PlatformServiceClient, FrontPageRequest, GameSearchRequest,
    },
    Session,
};
use tokio::sync::OnceCell;
use tonic::{transport::Channel, Request};

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
}

impl TaskRequest<PlatformManagerState> for PlatformRequest {
    async fn handle_request(
        self,
        sender: &tokio::sync::mpsc::Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<PlatformManagerState>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_platform_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            PlatformRequest::FrontPage(request) => match client.front_page(request).await {
                Ok(response) => sender
                    .try_send(TaskNotification::FrontPageResponse(response.into_inner()))
                    .unwrap(),
                Err(err) => println!("front page response err: {err}"),
            },
            PlatformRequest::Search(request) => todo!(),
        }
    }
}
