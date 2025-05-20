use rusqlite::{Connection, Result};

/// This will be a module for CRUD operations on the Hue Bridge.
///
struct Persistence {
    connection: Connection,
}

impl Persistence {
    /// Create a new Persistence instance.
    pub fn new() -> Self {
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        Persistence { connection }
    }

    pub fn check_table_version(&self) -> Resul<TableState> {}

    // pub fn upgrade_table(&self) {}

    pub fn create_table(&self) -> Result<TableState> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS lights (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                state TEXT NOT NULL
            )",
            [],
        )?;
        Ok(TableState::Exists)
    }
}
