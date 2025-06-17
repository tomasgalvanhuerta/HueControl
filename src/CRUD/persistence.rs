use super::table::TableWrapperError;
use super::table_existence::TableState;
use super::{auth_token::AuthToken, table::TableWrapper};
use rusqlite::Connection;
/// This will be a module for CRUD operations on the Hue Bridge.
pub struct Persistence {
    table_wrapper: TableWrapper,
}

impl Persistence {
    /// Create a new Persistence instance.
    pub fn new() -> Self {
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        let table_wrapper = TableWrapper::new(connection);
        Persistence { table_wrapper }
    }

    // Create a Queue to only have one operation at a time
    // 1. Create Queue and replace it with TableState
    // 2. Create method to
    // 3. Create Loop to complete the Queue

    pub fn create_table(&self) -> Result<TableState, TableWrapperError> {
        let table_wrapper = &self.table_wrapper;
        let table_result = table_wrapper.create_table();
        let table_result = table_result.map_err(|_| TableWrapperError::CouldNotCreateTable);
        return table_result;
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
}
