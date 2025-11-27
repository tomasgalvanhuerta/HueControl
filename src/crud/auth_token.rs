use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub clientkey: String,
    /// API Calls this User name
    pub username: String,
}

impl AuthToken {
    pub fn new(token: String, id: String) -> Self {
        AuthToken {
            clientkey: token,
            username: id,
        }
    }
}
