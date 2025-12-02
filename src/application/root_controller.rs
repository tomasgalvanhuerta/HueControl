use super::bridge_state::BridgeState;
use crate::communication::{discovery::Discovery, request_token::RequestToken};
use crate::crud::auth_token::AuthToken;
use crate::crud::persistence::Persistence;
use crate::crud::table_existence::TableState;
use crate::setup::hue_bridge::HueBridge;
use reqwest::Client;
pub struct RootController {
    persistence: Persistence,
    state: BridgeState,
}

impl RootController {
    pub fn new() -> Self {
        RootController {
            persistence: Persistence::new().expect("Unwrapping"),
            state: BridgeState::UnknownError,
        }
    }

    pub async fn start(&mut self) {
        // Confirm Table Exist, if not, create one
        let table_creation_result = self.persistence.create_table();
        match table_creation_result {
            Ok(table_state) => match table_state {
                TableState::DoesNotExist => {
                    // Program may not have access to create a new file
                    println!("Was not able to create folder, try again message");
                    println!("RootController state is {:?}", self.state)
                }
                TableState::Exists => {
                    println!("Table exists");
                    _ = self
                        .persistence
                        .check_token()
                        .map(|token| self.with_token(token))
                        .map_err(|_| self.search_bridge());
                }
            },
            Err(err) => println!("Error creating table: {:?}", err),
        }
    }

    pub fn with_token(&mut self, auth_token: AuthToken) {
        println!("Ready to send commands with token {:?}", auth_token);
        self.state = BridgeState::Connected(auth_token);
    }

    pub fn persist_auth_token(&mut self, auth_token: AuthToken) {
        println!("Persist_token with {:?}", auth_token);
        match &self.persistence.persist_auth_token(&auth_token) {
            Ok(_) => println!("Token persisted successfully"),
            Err(err) => println!("Error persisting token: {:?}", err),
        }
        self.with_token(auth_token);
    }

    async fn request_auth_token(&mut self, bridge: &HueBridge) {
        let client = Client::new();
        let ip_address = &bridge.ip_address;
        let request_token = RequestToken::request_token(&client, ip_address).await;
        match request_token {
            RequestToken::Success(auth_token) => {
                println!("Request token successful");
                self.with_token(auth_token);
            }
            RequestToken::Failed => {
                println!("Error requesting token");
                todo!("Present view to try establishing connection again")
            }
        }
    }

    async fn search_bridge(&mut self) {
        let discovered_bridge = Self::discover().await;
        println!("Discovered {:?} Hue Bridge", discovered_bridge.len());
        if discovered_bridge.len() == 1 {
            let bridge = discovered_bridge.iter().next().unwrap();
            self.request_auth_token(bridge).await;
        } else {
            todo!("Present multiple views")
        }
    }

    async fn discover() -> Vec<HueBridge> {
        let discovery = Discovery::new().await;
        match discovery {
            Discovery::Searching(_) => {
                println!("Searching For Hue Bridge");
                return Vec::new();
            }
            Discovery::FoundMultipleHue(hue_bridges, _) => {
                println!("Found multiple Bridhges {:?}", hue_bridges);
                // TODO: Implement logic to select a bridge from the list
                return hue_bridges;
            }
            Discovery::FoundSingleHue(hue_bridge, passed_client) => {
                let client = &passed_client;
                println!("Found Single Bridge {:?}", hue_bridge);
                Discovery::confirm_ip_address(&hue_bridge, client).await;
                Discovery::light_information(&hue_bridge.ip_address, client).await;
                return map_to_group(hue_bridge);
            }
            Discovery::Using(hue_bridge, _) => {
                println!("Going to Use {:?}", hue_bridge);
                return map_to_group(hue_bridge);
            }
        }
    }
}

fn map_to_group<T>(item: T) -> Vec<T> {
    let mut hue_bridge_container: Vec<T> = Vec::new();
    hue_bridge_container.push(item);
    return hue_bridge_container;
}
