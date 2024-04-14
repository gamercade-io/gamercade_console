use super::{TaskManager, TaskRequest};

// TODO: This
pub type ReleaseManager = TaskManager<ReleaseManagerState, ReleaseRequest>;

#[derive(Default)]
pub struct ReleaseManagerState {
    downloads: Vec<usize>,
}

pub enum ReleaseRequest {
    DownloadRelease,
}

impl TaskRequest<ReleaseManagerState> for ReleaseRequest {
    async fn handle_request(
        self,
        sender: &tokio::sync::mpsc::Sender<super::TaskNotification>,
        state: &tokio::sync::Mutex<ReleaseManagerState>,
    ) {
        todo!()
    }
}
