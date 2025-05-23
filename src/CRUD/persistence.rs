use rusqlite::{Connection, Result};

/// This will be a module for CRUD operations on the Hue Bridge.
///
struct Persistence {
    connection: Connection,
    table_state: TableState,
}

impl Persistence {
    /// Create a new Persistence instance.
    pub fn new() -> Self {
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        Persistence { connection }
    }

    pub fn check_table_version(&self) -> Result<TableState> {}

    // pub fn upgrade_table(&self) {}
}
