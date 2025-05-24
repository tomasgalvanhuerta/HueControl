use super::table_existence::TableState;
use super::{auth_token::AuthToken, table::TableWrapper};
use rusqlite::{Connection, Error, Result};
use tokio::sync::mpsc::OwnedPermit;
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

    pub fn check_token(connection: Connection) -> Result<AuthToken, rusqlite::Error> {
        let table = TableWrapper::new(connection)?;
        let table_result = table.read_table().map(|tokens| tokens.first());
        match table_result {
            Ok(token) => {
                if let Some(token.OwnedPermit) = token {
                    return Ok(Some(token));
                } else {
                    return Err(rusqlite::Error::ExecuteReturnedResults);
                }
            }
            Err(err) => Err(err.clone()),
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
