pub mod application;
pub mod communication;
pub mod crud;
pub mod setup;
pub use application::root_controller::RootController;
pub use crud::{persistence::Persistence, table_existence::TableState};
// use eframe::{NativeOptions, egui};
// use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    println!("Starting HueControl");
    let root_controller = RootController::new();
    root_controller.start().await;
}
