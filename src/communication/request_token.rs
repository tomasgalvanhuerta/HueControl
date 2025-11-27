use crate::crud::auth_token::AuthToken;

use super::request_token_body::RequestTokenBody;
use reqwest::{Client, Url};

pub enum RequestToken {
    Success(AuthToken),
    Failed,
}

impl RequestToken {
    /// Request for Token
    pub async fn request_token(client: &Client, ip_address: &str) -> RequestToken {
        let url = format!("https://{}/api", ip_address);
        let url = Url::parse(&url);
        match url {
            Ok(url) => {
                let token = Self::post(url, client).await;
                token
            }
            Err(error) => {
                println!("Unable to fetch token with result {:?}", error);
                RequestToken::Failed
            }
        }
    }

    async fn post(url: Url, client: &Client) -> RequestToken {
        // Convert it to be generic
        let device_name = String::from("Rust Project");
        let get_result = client
            .post(url)
            .body(RequestTokenBody::DeviceType(device_name).as_string())
            .body(RequestTokenBody::GenerateClientKey(true).as_string())
            .send()
            .await;
        match get_result {
            Ok(response) => {
                let response_text = response.text().await.unwrap_or("Unknown".to_string());
                println!("RecievedToken for {:?}", response_text);
                // RequestToken::Success(response_text)
                RequestToken::Failed
            }
            Err(error) => {
                println!("Unable to recieve reqeust from {:?}", error);
                RequestToken::Failed
            }
        }
    }
}
