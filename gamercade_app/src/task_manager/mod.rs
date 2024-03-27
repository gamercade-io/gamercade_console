use std::sync::Arc;

use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};

mod tags;
pub use tags::*;

mod authors;
pub use authors::*;

mod super_task_manager;
pub use super_task_manager::*;

const SUPER_TASK_CHANNEL_SIZE: usize = 256;
const TASK_CHANNEL_LENGTH: usize = 8;

pub struct TaskManager<STATE, REQUEST> {
    pub state: Arc<Mutex<STATE>>,
    sender: Sender<REQUEST>,
}

impl<STATE, REQUEST> TaskManager<STATE, REQUEST>
where
    STATE: Default + Send + 'static,
    REQUEST: Send + 'static + TaskRequest<STATE>,
{
    pub fn new(notification_tx: Sender<TaskNotification>) -> Self {
        let state = Arc::new(Mutex::new(STATE::default()));
        Self {
            sender: Self::spawn_task(notification_tx, state.clone()),
            state,
        }
    }

    pub fn send_request(&self, request: REQUEST) {
        if let Err(e) = self.sender.try_send(request) {
            panic!("send_request failed {e}")
        }
    }

    fn spawn_task(
        notification_tx: Sender<TaskNotification>,
        state: Arc<Mutex<STATE>>,
    ) -> Sender<REQUEST> {
        let (client_sender, mut receiver) = channel::<REQUEST>(TASK_CHANNEL_LENGTH);

        tokio::spawn(async move {
            let state = state.clone();
            while let Some(request) = receiver.recv().await {
                request.handle_request(&notification_tx, &state).await
            }
        });

        client_sender
    }
}

pub trait TaskRequest<STATE: Send> {
    fn handle_request(
        self,
        sender: &Sender<TaskNotification>,
        state: &Mutex<STATE>,
    ) -> impl std::future::Future<Output = ()> + Send;
}
