use gamercade_interface::{
    auth::{
        auth_service_client::AuthServiceClient, login_request::Provider, LoginRequest,
        SignUpRequest,
    },
    Session,
};
use tokio::sync::{mpsc::Sender, Mutex, OnceCell};
use tonic::transport::Channel;

use crate::urls::SERVICE_IP_GRPC;

use super::{TaskManager, TaskNotification, TaskRequest};

#[derive(Default, Debug)]
pub enum AuthState {
    // Default State
    #[default]
    Unauthorized,

    // Holding Session
    SessionHeld(Session),
}

impl AuthState {
    pub fn get_session(&self) -> Option<Session> {
        match self {
            AuthState::Unauthorized => None,
            AuthState::SessionHeld(session) => Some(session.clone()),
        }
    }
}

pub type AuthManager = TaskManager<AuthManagerState, AuthRequest>;

#[derive(Default)]
pub struct AuthManagerState {
    client: OnceCell<AuthServiceClient<Channel>>,
}

async fn init_auth_client() -> AuthServiceClient<Channel> {
    AuthServiceClient::connect(SERVICE_IP_GRPC).await.unwrap()
}

pub enum AuthRequest {
    Login(LoginRequest),
    SignUp(SignUpRequest),
}

impl TaskRequest<AuthManagerState> for AuthRequest {
    async fn handle_request(
        self,
        sender: &Sender<TaskNotification>,
        state: &Mutex<AuthManagerState>,
    ) {
        let mut lock = state.lock().await;
        lock.client.get_or_init(init_auth_client).await;
        let client = lock.client.get_mut().unwrap();

        match self {
            AuthRequest::Login(login) => handle_login(login, client, sender).await,
            AuthRequest::SignUp(sign_up) => handle_sign_up(sign_up, client, sender).await,
        }
    }
}

async fn handle_login(
    request: LoginRequest,
    client: &mut AuthServiceClient<Channel>,
    sender: &Sender<TaskNotification>,
) {
    println!("Trying to login...");
    match client.login(request).await {
        Ok(response) => {
            let response = response.into_inner();

            if let Ok(session) = Session::try_from(response.session.as_slice()) {
                sender
                    .send(TaskNotification::AuthStateChanged(AuthState::SessionHeld(
                        session,
                    )))
                    .await
                    .unwrap();
            } else {
                // TODO: Handle this
                println!("Error parsing session from server")
            }
        }
        Err(e) => {
            sender.send(TaskNotification::LoginFailed).await.unwrap();
            println!("{e}");
        }
    }
}

async fn handle_sign_up(
    request: SignUpRequest,
    client: &mut AuthServiceClient<Channel>,
    sender: &Sender<TaskNotification>,
) {
    println!("Trying to sign up...");
    match client.sign_up(request).await {
        Ok(response) => {
            let response = response.into_inner();

            if let Ok(session) = Session::try_from(response.session.as_slice()) {
                sender
                    .send(TaskNotification::AuthStateChanged(AuthState::SessionHeld(
                        session,
                    )))
                    .await
                    .unwrap();
            } else {
                // TODO: Handle this
                println!("Error parsing session from server")
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

impl AuthManager {
    pub fn try_login(&self, username: &str, password: &str) {
        if let Err(_e) = self.sender.try_send(AuthRequest::Login(LoginRequest {
            provider: Some(Provider::Username(username.to_string())),
            password: password.to_string(),
        })) {
            panic!("Couldn't send login request over channel.");
        };
    }

    pub fn try_register(&self, username: &str, email: &str, password: &str) {
        if let Err(_e) = self.sender.try_send(AuthRequest::SignUp(SignUpRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        })) {
            panic!("Couldn't send login request over channel.");
        };
    }
}
