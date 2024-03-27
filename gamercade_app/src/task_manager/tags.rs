use gamercade_interface::{common::Empty, tag::tag_service_client::TagServiceClient};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::{transport::Channel, Request};

use crate::{
    ips::SERVICE_IP,
    local_directory::{Tag, TagId},
};

use super::{TaskManager, TaskNotification, TaskRequest};

pub type TagManager = TaskManager<TagManagerState, TagRequest>;

#[derive(Default)]
pub struct TagManagerState {
    client: OnceCell<TagServiceClient<Channel>>,
}

async fn init_tag_client() -> TagServiceClient<Channel> {
    TagServiceClient::connect(SERVICE_IP).await.unwrap()
}

impl TaskRequest<TagManagerState> for TagRequest {
    async fn handle_request(
        self,
        notification_tx: &Sender<TaskNotification>,
        state: &Mutex<TagManagerState>,
    ) {
        match self {
            TagRequest::Initialize => {
                let lock = state.lock().await;
                let response = lock
                    .client
                    .get_or_init(init_tag_client)
                    .await
                    .clone()
                    .get_global_tags(Request::new(Empty {}))
                    .await
                    .unwrap();

                let mut response = response.into_inner();
                let tags = response
                    .tags
                    .drain(..)
                    .map(|tag| (TagId(tag.pid as usize), Tag(tag.name)))
                    .collect();
                let message = TaskNotification::GlobalTags(tags);
                notification_tx.send(message).await.unwrap();
            }
        }
    }
}

pub enum TagRequest {
    Initialize,
}
