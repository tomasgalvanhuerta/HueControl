use reqwest::{Client, Url};

enum RequestToken {}

impl RequestToken {
    /// Request for Token
    pub async fn request_token(client: &Client, ip_address: &str) {
        let url = format!("https://{}/api", ip_address.clone());
        let url = Url::parse(&url)
            .map_err(|error| println!("Unable to fetch token with result {:?}", error))
            .unwrap();
    }

    async fn get<T>(url: Url, client: &Client) -> String {
        let get_result = client.get(url).send().await;
        match get_result {
            Ok(response) => {
                let response_text = response.text().await.unwrap_or("Unknown".to_string());
                println!("RecievedToken for {:?}", response_text);
                response_text
            }
            Err(error) => {
                println!("Unable to recieve reqeust from {:?}", error);
                "".to_string()
            }
        }
    }
}
