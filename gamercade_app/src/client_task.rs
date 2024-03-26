// use std::{marker::PhantomData, sync::Arc};

// use tokio::sync::{
//     mpsc::{channel, Sender},
//     Mutex,
// };

// pub struct ClientTask<STATE, REQUEST, HANDLER> {
//     pub state: Arc<Mutex<STATE>>,
//     pub sender: Sender<REQUEST>,
//     handler: PhantomData<HANDLER>,
// }

// impl<STATE, REQUEST, HANDLER> Default for ClientTask<STATE, REQUEST, HANDLER>
// where
//     STATE: Default + Send + 'static,
//     REQUEST: Default + Send + 'static,
//     HANDLER: ClientTaskHandler<STATE, REQUEST> + Default + Send + 'static,
// {
//     fn default() -> Self {
//         let state = Arc::new(Mutex::new(STATE::default()));
//         Self {
//             sender: Self::spawn_task(state.clone()),
//             state,
//             handler: PhantomData::default(),
//         }
//     }
// }

// impl<STATE, REQUEST, HANDLER> ClientTask<STATE, REQUEST, HANDLER>
// where
//     STATE: Send + 'static,
//     REQUEST: Send + 'static,
//     HANDLER: ClientTaskHandler<STATE, REQUEST> + Send,
// {
//     fn spawn_task(state: Arc<Mutex<STATE>>) -> Sender<REQUEST> {
//         let (client_sender, mut receiver) = channel(4);

//         tokio::spawn(async move {
//             let state = state.clone();
//             while let Some(request) = receiver.recv().await {
//                 HANDLER::handle_request(&state, request).await
//             }
//         });

//         client_sender
//     }
// }

// pub trait ClientTaskHandler<STATE, REQUEST> {
//     fn handle_request(
//         state: &Arc<Mutex<STATE>>,
//         request: REQUEST,
//     ) -> impl std::future::Future<Output = ()> + Send;
// }
