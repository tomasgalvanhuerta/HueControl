struct AuthToken {
    date: DateTime<Utc>,
    token: String,
}

impl AuthToken {
    pub fn new(token: String) -> Self {
        AuthToken { token }
    }
}
