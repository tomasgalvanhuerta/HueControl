use super::hue_bridge::HueBridge;
use super::hue_call_dns::get_dns_data;
use reqwest::Client;

pub struct HueSetup {
    client: Client,
}

impl HueSetup {
    pub fn new(client: Client) -> Self {
        HueSetup { client: client }
    }

    pub async fn start(&self) -> Result<Vec<HueBridge>, SetupHueError> {
        let hue_bridge_response = self.hue_bridge().await;
        match &hue_bridge_response {
            Ok(bridge) => println!("Bridge Found, {:?}", bridge),
            Err(err) => println!("Does not exist with error"),
        }
        return hue_bridge_response;
    }

    async fn hue_bridge(&self) -> Result<Vec<HueBridge>, SetupHueError> {
        // get bridge Information
        // Client is ARC<Client>
        // Creating a Clone creates a new pointer and adds +1 to ARC count
        let client = self.client.clone();
        let hue_bridge_string = get_dns_data(client)
            .await
            .map_err(|error| SetupHueError::NoCall)?;
        println!("Bridge info {hue_bridge_string}");
        let hue_bridge_response =
            HueBridge::from_json(hue_bridge_string).map_err(|error| SetupHueError::NoCall);
        return hue_bridge_response;
    }
}

#[derive(Debug)]
pub enum SetupHueError {
    NoCall,
    DNSError,
    BridgeError,
    BridgeNotFound,
}
