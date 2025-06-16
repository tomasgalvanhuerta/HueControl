use std::time::Duration;

pub struct AuthToken {
    pub time_interval: Duration,
    pub token: String,
    pub id: String,
}

impl AuthToken {
    pub fn new(duration: Duration, token: String, id: String) -> Self {
        AuthToken {
            time_interval: duration,
            token,
            id,
        }
    }
}
