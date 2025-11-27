use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub clientkey: String,
    /// API Calls this User name
    pub username: String,
}

impl AuthToken {
    pub fn new(duration: Duration, token: String, id: String) -> Self {
        AuthToken {
            clientkey: token,
            username: id,
        }
    }
}
