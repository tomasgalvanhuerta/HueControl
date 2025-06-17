use crate::crud::persistence::Persistence;
use reqwest::{Client, Url};

enum RequestToken {}

impl RequestToken {
    /// Request for Token
    fn request_token(&self, persistance: &Persistence, client: &Client, ip_address: &str) {
        let url = format!("https://{}/api", ip_address.clone());
        let mut url = Url::parse(&url).unwrap(); // BANG!
    }
}
