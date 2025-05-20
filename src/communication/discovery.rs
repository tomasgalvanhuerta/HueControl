use super::super::setup::setup_hue::HueSetup;
use crate::setup::hue_bridge::HueBridge;
use reqwest::{Client, Url};

pub enum Discovery {
    Searching(Client),
    FoundMultipleHue(Vec<HueBridge>, Client),
    FoundSingleHue(HueBridge, Client),
    Using(HueBridge, Client),
}

impl Discovery {
    pub async fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Was not able to build a new Client");
        let setup = HueSetup::new(client.clone());
        match setup.start().await {
            Ok(hue_bridge) => {
                println!("Found hue Bridge {:?}", hue_bridge);
                match hue_bridge.len() {
                    0 => Self::Searching(client.clone()),
                    1 => {
                        if let Some(first) = hue_bridge.into_iter().next() {
                            Self::FoundSingleHue(first, client)
                        } else {
                            Self::Searching(client)
                        }
                    }
                    n if n > 1 => Self::FoundMultipleHue(hue_bridge, client),
                    _ => Self::Searching(client),
                }
            }
            Err(error) => {
                print!("Error found fetching Hue Bridge {:?}", error);
                Self::Searching(client)
            }
        }
    }

    /// Confirm the searching Hue Bridge can accept commands
    pub async fn confirm_ip_address(&self, hue_bridge: &HueBridge, client: &Client) {
        let url = format!(
            "https://{}/api/0/config",
            hue_bridge.internalipaddress.clone()
        );
        let mut url = Url::parse(&url).expect("Was not able to parte URL");
        url.set_port(Option::Some(443_u16))
            .expect("Not able to port to 443");
        println!("Url is {:?}", url);
        let content = client.get(url).send().await.expect("It was not possible");
        println!("text: {content:?}");
        // return hue_bridge;
    }

    /// Get Room information
    pub async fn light_information(&self, ip_address: &String, client: &Client) {
        let url = format!("https://{}/clip/v2/resource/light", ip_address.clone());
        let mut url = Url::parse(&url).expect("Was not able to construct Room search URL");
        url.set_port(Option::Some(443_u16))
            .expect("Not able to port to 443");
        println!("Url is {:?}", url);
        let response = client
            .get(url)
            .send()
            .await
            .expect("Client was not able to search lights");
        println!("Response: {response:?}");
    }

    /// Loop Search until something is found
    pub async fn search_ForBrige(&self) {}
}
