use std::sync::Arc;

use gamercade_interface::auth::{
    auth_service_client::AuthServiceClient, login_request::Provider, LoginRequest,
    RefreshTokenRequest, SignUpRequest,
};
use tokio::{
    select,
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
};
use tonic::transport::Channel;

use crate::{auth::auth_state::AuthToken, ips::AUTH_IP};

use super::auth_state::AuthState;

pub struct AuthClient {
    pub state: Arc<RwLock<AuthState>>,
    sender: Sender<AuthClientRequest>,
}

pub enum AuthClientRequest {
    Login(LoginRequest),
    SignUp(SignUpRequest),
    RefreshToken(RefreshTokenRequest),
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
        if let Err(_e) = self.sender.try_send(AuthClientRequest::Login(LoginRequest {
            provider: Some(Provider::Username(username.to_string())),
            password: password.to_string(),
        })) {
            panic!("Couldn't send login request over channel.");
        };
    }

    pub fn try_register(&self, username: &str, email: &str, password: &str) {
        if let Err(_e) = self
            .sender
            .try_send(AuthClientRequest::SignUp(SignUpRequest {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            }))
        {
            panic!("Couldn't send login request over channel.");
        };
    }
}

fn spawn_task(auth_state: Arc<RwLock<AuthState>>) -> Sender<AuthClientRequest> {
    let (auth_client_sender, rx) = channel(4);

    tokio::spawn(async move { AuthTask::new(rx, auth_state).run().await });

    auth_client_sender
}

struct AuthTask {
    main_thread_receiver: Receiver<AuthClientRequest>,
    auth_state: Arc<RwLock<AuthState>>,
}

impl AuthTask {
    fn new(
        main_thread_receiver: Receiver<AuthClientRequest>,
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
                // Handle Requests
                Some(request) = self.main_thread_receiver.recv() => {
                    match request {
                        AuthClientRequest::Login(login) => self.handle_login(&mut client, login).await,
                        AuthClientRequest::SignUp(signup) => self.handle_sign_up(&mut client, signup).await,
                        AuthClientRequest::RefreshToken(refresh) => self.handle_refresh(&mut client, refresh).await,
                    }
                }
            }
        }
    }

    async fn handle_login(
        &mut self,
        client: &mut AuthServiceClient<Channel>,
        request: LoginRequest,
    ) {
        println!("Trying to login...");
        match client.login(request).await {
            Ok(response) => {
                let response = response.into_inner();
                let mut write = self.auth_state.write().await;
                *write = AuthState::TokensHeld(AuthToken {
                    access_token: response.access_token,
                    refresh_token: response.refresh_token,
                    expires_at: response.expires_at,
                });
                // TODO: Update the login page / move to browsing
                println!("Logged in successfully: {:?}", write);
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    async fn handle_sign_up(
        &mut self,
        client: &mut AuthServiceClient<Channel>,
        request: SignUpRequest,
    ) {
        println!("Trying to sign up...");
        match client.sign_up(request).await {
            Ok(_) => {
                // TODO: Update the login page / move to Login
                println!("Signed up successfully.");
            }
            Err(e) => {
                // TODO:
                println!("{e}");
            }
        }
    }

    async fn handle_refresh(
        &mut self,
        client: &mut AuthServiceClient<Channel>,
        request: RefreshTokenRequest,
    ) {
        todo!()
    }
}
