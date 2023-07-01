#[derive(Default, Debug)]
pub enum AuthState {
    // Default State
    #[default]
    Unauthorized,

    // Holding Tokens
    TokensHeld(AuthToken),
}

#[derive(Debug)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
}
