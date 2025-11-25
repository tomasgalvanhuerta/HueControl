use eframe::egui::TextBuffer;
use reqwest::{Client, Request, Url};

enum RequestToken {
    Success(String),
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
        let deviceName = String::from("Rust Project");
        let get_result = client
            .post(url)
            .body(RequestTokenBody::devicetype(deviceName).as_string())
            .body(RequestTokenBody::generateclientkey(true).as_string())
            .send()
            .await;
        match get_result {
            Ok(response) => {
                let response_text = response.text().await.unwrap_or("Unknown".to_string());
                println!("RecievedToken for {:?}", response_text);
                RequestToken::Success(response_text)
            }
            Err(error) => {
                println!("Unable to recieve reqeust from {:?}", error);
                RequestToken::Failed
            }
        }
    }
}

enum RequestTokenBody {
    devicetype(String),
    generateclientkey(bool),
}

impl RequestTokenBody {
    fn as_string(&self) -> String {
        match self {
            RequestTokenBody::devicetype(name) => String::from("devicetype:").push_str(name),
            RequestTokenBody::generateclientkey(flag) => String::from("generateclientkey:{flag}"),
        }
    }
}
