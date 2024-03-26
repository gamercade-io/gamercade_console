use gamercade_interface::{common::Empty, tag::tag_service_client::TagServiceClient};
use tokio::sync::{Mutex, OnceCell};
use tonic::{transport::Channel, Request};

use crate::ips::SERVICE_IP;

use super::{TaskManager, TaskRequest};

pub type TagManager = TaskManager<TagManagerState, TagRequest>;

#[derive(Default)]
pub struct TagManagerState {
    client: OnceCell<TagServiceClient<Channel>>,
}

async fn init_tag_client() -> TagServiceClient<Channel> {
    TagServiceClient::connect(SERVICE_IP).await.unwrap()
}

impl TaskRequest<TagManagerState> for TagRequest {
    async fn handle_request(self, state: &Mutex<TagManagerState>) {
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
                let response = response.into_inner();
                response
                    .tags
                    .iter()
                    .for_each(|tag| println!("Got tag: {}: {}", tag.pid, tag.name))
            }
        }
    }
}

pub enum TagRequest {
    Initialize,
}
