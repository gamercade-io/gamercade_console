use std::sync::Arc;

use gamercade_interface::auth::{auth_service_client::AuthServiceClient, LoginRequest};
use tokio::{
    select,
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
};

use crate::{auth::auth_state::AuthToken, ips::AUTH_IP};

use super::auth_state::AuthState;

pub struct AuthClient {
    pub state: Arc<RwLock<AuthState>>,
    sender: Sender<LoginRequest>,
}

impl Default for AuthClient {
    fn default() -> Self {
        let state = Arc::new(RwLock::new(AuthState::Unauthorized));
        Self {
            sender: spawn_task(state.clone()),
            state,
        }
    }
}

impl AuthClient {
    /// Asynchronously sends a login request to the Auth thread
    pub fn try_login(&self, username: &str, password: &str) {
        if let Err(_e) = self.sender.try_send(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        }) {
            panic!("Couldn't send login request over channel.");
        };
    }
}

fn spawn_task(auth_state: Arc<RwLock<AuthState>>) -> Sender<LoginRequest> {
    let (auth_client_sender, rx) = channel(4);

    tokio::spawn(async move { AuthTask::new(rx, auth_state).run().await });

    auth_client_sender
}

struct AuthTask {
    main_thread_receiver: Receiver<LoginRequest>,
    auth_state: Arc<RwLock<AuthState>>,
}

impl AuthTask {
    fn new(
        main_thread_receiver: Receiver<LoginRequest>,
        auth_state: Arc<RwLock<AuthState>>,
    ) -> Self {
        Self {
            main_thread_receiver,
            auth_state,
        }
    }

    async fn run(mut self) -> ! {
        let mut client = AuthServiceClient::connect(AUTH_IP).await.unwrap();

        loop {
            select! {
                // Handle Login Requests
                Some(login) = self.main_thread_receiver.recv() => {
                    match client.login(LoginRequest {
                        username: login.username,
                        password: login.password,
                    }).await {
                        Ok(response) => {
                            println!("Trying to login...");
                            let response = response.into_inner();
                            let mut write = self.auth_state.write().await;
                            *write = AuthState::TokensHeld(AuthToken {
                                access_token: response.access_token,
                                refresh_token: response.refresh_token,
                                expires_at: response.expires_at,
                            });
                            println!("Logged in successfully: {:?}", write);
                        },
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }

                // TODO:
                // Handle Refresh Requests
            }
        }
    }
}
