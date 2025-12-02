use crate::crud::auth_token::AuthToken;

#[derive(Debug)]
pub enum BridgeState {
    NotRegistered,
    Connected(AuthToken),
    UnknownError,
}

impl BridgeState {
    pub fn new() -> Self {
        BridgeState::NotRegistered
    }

    pub fn start_with_token(token: AuthToken) -> Self {
        BridgeState::Connected(token)
    }
}
