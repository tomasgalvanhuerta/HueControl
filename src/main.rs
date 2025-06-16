pub mod communication;
pub mod crud;
pub mod setup;
pub mod setup_mdns;

use crud::persistence::Persistence;
use std::{thread::sleep, time::Duration};

use communication::discovery::Discovery;

#[tokio::main]
async fn main() {
    loop {
        check_table();
        discover();
        sleep(Duration::from_secs(5));
    }
}

async fn check_table() {
    let persistence = Persistence::new();

    // Confirm Table Exist, if not, create one
    persistence.create_table();
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
