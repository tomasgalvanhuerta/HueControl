use super::table_existence::TableState;
// use crate::crud::table_existence::TableState;
use rusqlite::{Connection, Error, Result};
/// This will be a module for CRUD operations on the Hue Bridge.
struct Persistence {
    connection: Connection,
    table_state: TableState,
}

impl Persistence {
    /// Create a new Persistence instance.
    pub fn new() -> Self {
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        let table_state = TableState::Create;
        Persistence {
            connection,
            table_state,
        }
    }

    pub fn check_table_version(&self) -> Result<TableState, rusqlite::Error> {
        // let table_state = TableState::new(&self.connection)?;
        // Ok(table_state)
        // warning fix me!
        Err(Error::ExecuteReturnedResults)
    }

    // pub fn upgrade_table(&self) {}
}
