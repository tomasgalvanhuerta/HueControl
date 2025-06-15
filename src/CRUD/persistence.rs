use super::table_existence::TableState;
use super::{auth_token::AuthToken, table::TableWrapper};
use rusqlite::Connection;
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

    pub fn check_token(connection: Connection) -> Option<AuthToken> {
        // Crash safer
        let table = TableWrapper::new(connection).expect("TableWrapper not able to be created");
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

enum PersistanceError {}
