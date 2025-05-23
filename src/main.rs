pub mod communication;
pub mod crud;
pub mod setup;
pub mod setup_mdns;

use communication::discovery::{self, Discovery};

#[tokio::main]
async fn main() {
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
