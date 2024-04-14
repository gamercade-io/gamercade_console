use gamercade_interface::{author::author_service_client::AuthorServiceClient, common::Empty};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::{transport::Channel, Request};

use crate::{
    local_directory::{PermissionLevel, PermissionLevelId},
    urls::SERVICE_IP_GRPC,
};

use super::{TaskManager, TaskNotification, TaskRequest};

pub type AuthorManager = TaskManager<AuthorManagerState, AuthorRequest>;

#[derive(Default)]
pub struct AuthorManagerState {
    client: OnceCell<AuthorServiceClient<Channel>>,
}

async fn init_author_client() -> AuthorServiceClient<Channel> {
    AuthorServiceClient::connect(SERVICE_IP_GRPC).await.unwrap()
}

impl TaskRequest<AuthorManagerState> for AuthorRequest {
    async fn handle_request(
        self,
        notification_tx: &Sender<TaskNotification>,
        state: &Mutex<AuthorManagerState>,
    ) {
        match self {
            AuthorRequest::Initialize => {
                let lock = state.lock().await;
                let response = lock
                    .client
                    .get_or_init(init_author_client)
                    .await
                    .clone()
                    .get_global_permission_levels(Request::new(Empty {}))
                    .await
                    .unwrap();

                let mut response = response.into_inner();
                let levels = response
                    .levels
                    .drain(..)
                    .map(|tag| {
                        (
                            PermissionLevelId(tag.level_id as usize),
                            PermissionLevel {
                                name: tag.level_name,
                                strength: tag.level_strength as i32,
                            },
                        )
                    })
                    .collect();
                let message = TaskNotification::GlobalPermissionLevels(levels);
                notification_tx.send(message).await.unwrap();
            }
        }
    }
}

pub enum AuthorRequest {
    Initialize,
}
