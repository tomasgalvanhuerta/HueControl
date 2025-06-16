use super::table::{self, TableWrapperError};
use super::table_existence::TableState;
use super::{auth_token::AuthToken, table::TableWrapper};
use rusqlite::Connection;
/// This will be a module for CRUD operations on the Hue Bridge.
pub struct Persistence {
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

    // Create a Queue to only have one operation at a time
    // 1. Create Queue and replace it with TableState
    // 2. Create method to
    // 3. Create Loop to complete the Queue

    pub fn create_table(&self) -> Result<TableState, TableWrapperError> {
        let connection = &self.connection;
        let table_wrapper = table::TableWrapper::new(connection);
        return Ok(TableState::Create);
    }

    pub fn check_token(connection: Connection) -> Option<AuthToken> {
        // Crash safer
        let table = TableWrapper::new(connection);
        let read_tables = table.read_table();
        let single_token = read_tables
            .into_iter()
            .next()
            .and_then(|vec| vec.into_iter().next());
        if let Some(token) = single_token {
            return Some(token);
        }
        return None;
    }

    // pub fn check_table_version(&self) -> Result<TableState, rusqlite::Error> {
    //     // let table_state = TableState::new(&self.connection)?;
    //     // Ok(table_state)
    //     // warning fix me!
    //     Err(Error::ExecuteReturnedResults)
    // }

    // pub fn upgrade_table(&self) {}
}
