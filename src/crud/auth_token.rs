use std::time::Duration;

pub struct AuthToken {
    pub time_interval: Duration,
    pub token: String,
}

impl AuthToken {
    pub fn new(duration: Duration, token: String) -> Self {
        AuthToken {
            time_interval: duration,
            token,
        }
    }
}
