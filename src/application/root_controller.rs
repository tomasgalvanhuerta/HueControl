use crate::communication::discovery::Discovery;
use crate::crud::persistence::Persistence;
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
                TableState::Create => {
                    println!("Created a new Table");
                    Self::discover().await;
                }
                TableState::DoesNotExist => println!("Table Could not be created"),
                TableState::Exists => {
                    let token_result = persistence.check_token();
                    println!("Table already existed {:?}", token_result)
                }
            },
            Err(err) => println!("Error creating table"),
        }
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
