use crate::communication::discovery::Discovery;
use crate::crud::auth_token::AuthToken;
use crate::crud::persistence::Persistence;
use crate::crud::table::TableWrapper;
use crate::crud::table_existence::TableState;
use crate::setup::hue_bridge::HueBridge;
pub struct RootController {}

impl RootController {
    pub fn new() -> Self {
        RootController {}
    }

    pub async fn start(&self) {
        let persistence = Persistence::new().expect("Unwrapping");

        // Confirm Table Exist, if not, create one
        let table_creation_result = persistence.create_table();
        match table_creation_result {
            Ok(table_state) => match table_state {
                TableState::DoesNotExist => {
                    // Program may not have access to create a new file
                    println!("Was not able to create folder, try again message");
                }
                TableState::Exists => {
                    println!("Table exists");
                    let token_result = &persistence
                        .check_token()
                        .map(|token| Self::with_token(token))
                        .map_err(|_| Self::search_bridge());
                }
            },
            Err(err) => println!("Error creating table"),
        }
    }

    pub fn with_token(auth_token: AuthToken) {
        println!("With token {:?}", auth_token);
    }

    pub fn persist_bridge(bridge: HueBridge) {
        println!("Persist_bridge with {:?}", bridge);
        // let auth_token = AuthToken::new(duration, bridge.token, bridge.id);
        // let persistence = Persistence::new(); // Pass it here
        // persistence.persist_bridge(bridge);
    }

    fn request_auth_token(bridge: HueBridge) {}

    fn search_bridge() {
        let discovered_bridge = Self::discover();
        println!("Discovering Hue Bridge");
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
                Discovery::light_information(&hue_bridge.internalipaddress, client).await;
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
