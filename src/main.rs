pub mod communication;
pub mod crud;
pub mod setup;

use crud::{persistence::Persistence, table_existence::TableState};
use eframe::{NativeOptions, egui};
use std::{thread::sleep, time::Duration};

use communication::discovery::Discovery;

#[tokio::main]
async fn main() {
    println!("Starting HueControl");
    start().await;
}

async fn start() {
    let persistence = Persistence::new();

    // Confirm Table Exist, if not, create one
    let table_creation_result = persistence.create_table();
    match table_creation_result {
        Ok(table_state) => match table_state {
            TableState::Create => {
                println!("Created a new Table");
                discover().await;
            }
            TableState::DoesNotExist => println!("Table Could not be created"),
            TableState::Exists => {
                discover().await;
                println!("Table already existed")
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
