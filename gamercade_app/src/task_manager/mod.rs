use std::sync::Arc;

use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};

mod tags;
pub use tags::*;

pub struct TaskManager<STATE, REQUEST> {
    pub state: Arc<Mutex<STATE>>,
    sender: Sender<REQUEST>,
}

impl<STATE, REQUEST> Default for TaskManager<STATE, REQUEST>
where
    STATE: Default + Send + 'static,
    REQUEST: Send + 'static + TaskRequest<STATE>,
{
    fn default() -> Self {
        let state = Arc::new(Mutex::new(STATE::default()));
        Self {
            sender: Self::spawn_task(state.clone()),
            state,
        }
    }
}

impl<STATE, REQUEST> TaskManager<STATE, REQUEST>
where
    STATE: Send + 'static,
    REQUEST: Send + 'static + TaskRequest<STATE>,
{
    pub fn send_request(&self, request: REQUEST) {
        if let Err(e) = self.sender.try_send(request) {
            panic!("send_request failed {e}")
        }
    }

    fn spawn_task(state: Arc<Mutex<STATE>>) -> Sender<REQUEST> {
        let (client_sender, mut receiver) = channel::<REQUEST>(4);

        tokio::spawn(async move {
            let state = state.clone();
            while let Some(request) = receiver.recv().await {
                request.handle_request(&state).await
            }
        });

        client_sender
    }
}

pub trait TaskRequest<STATE: Send> {
    fn handle_request(self, state: &Mutex<STATE>) -> impl std::future::Future<Output = ()> + Send;
}
