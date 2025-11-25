use crate::crud::auth_token::AuthToken;
use crate::crud::{
    table::TableWrapper, table_existence::TableState, table_wrapper_error::TableWrapperError,
};
use rusqlite::Connection;

/// This will be a module for CRUD operations on the Hue Bridge.
pub struct Persistence {
    table_wrapper: TableWrapper,
}

impl Persistence {
    /// Create a new Persistence instance.
    pub fn new() -> Result<Self, rusqlite::Error> {
        let path = "./huedb.db3";
        let connection_result = Connection::open(path);
        match connection_result {
            Ok(connection) => {
                let table_wrapper = TableWrapper::new(connection);
                return Ok(Persistence { table_wrapper });
            }
            Err(error) => {
                println!("Error {}", error);
                return Err(error);
            }
        }
    }

    // Create a Queue to only have one operation at a time
    // 1. Create Queue and replace it with TableState
    // 2. Create method to
    // 3. Create Loop to complete the Queue

    pub fn create_table(&self) -> Result<TableState, TableWrapperError> {
        let table_wrapper = &self.table_wrapper;
        let table_result = table_wrapper
            .open_or_create_table()
            .map(|_| TableState::Exists);
        let table_result = table_result.map_err(|_| TableWrapperError::CouldNotCreateTable);
        return table_result;
    }

    pub fn check_token(&self) -> Result<AuthToken, TableWrapperError> {
        // Does not belong here
        // Crash safer
        let read_tables = self.table_wrapper.read_table();
        let single_token = read_tables
            .into_iter()
            .next()
            .and_then(|vec| vec.into_iter().next());
        if let Some(token) = single_token {
            return Ok(token);
        }
        return Err(TableWrapperError::CouldNotFindToken);
    }
}
