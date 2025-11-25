use crate::communication::discovery::Discovery;
use crate::crud::auth_token::AuthToken;
use crate::crud::persistence::Persistence;
use crate::crud::table::TableWrapper;
use crate::crud::table_existence::TableState;
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
                        .map_err(|_| Self::request_auth_token());
                }
            },
            Err(err) => println!("Error creating table"),
        }
    }

    pub fn with_token(auth_token: AuthToken) {}

    async fn request_auth_token() {
        Self::discover().await;
    }

    async fn discover() {
        let discovery = Discovery::new().await;
        match &discovery {
            Discovery::Searching(_) => println!("Searching For Hue Bridge"),
            Discovery::FoundMultipleHue(hue_bridges, _) => {
                println!("Found multiple Bridhges {:?}", hue_bridges)
            }
            Discovery::FoundSingleHue(hue_bridge, passed_client) => {
                println!("Found Single Bridge {:?}", hue_bridge);
                discovery
                    .confirm_ip_address(hue_bridge, passed_client)
                    .await;
                discovery
                    .light_information(&hue_bridge.internalipaddress, passed_client)
                    .await;
            }
            Discovery::Using(hue_bridge, _) => {
                println!("Going to Use {:?}", hue_bridge)
            }
        }
    }
}
