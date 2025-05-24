use std::time::Duration;

use super::auth_token::AuthToken;
use super::table_existence::TableState;
use rusqlite::{Connection, Result};

struct TableWrapper {
    connection: Connection,
}

impl TableWrapper {
    pub fn create_table(&self) -> Result<TableState> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS authToken (
                token TEXT NOT NULL
            )",
            [],
        )?;
        Ok(TableState::Exists)
    }

    pub fn read_table(&self) -> Result<Vec<AuthToken>> {
        let mut stmt = self
            .connection
            .prepare("SELECT date, token FROM authToken")?;
        let rows = stmt.query_map([], |row| {
            Ok(AuthToken {
                time_interval: Duration::from_secs(row.get(0)?),
                token: row.get(1)?,
            })
        })?;

        let mut auth_tokens = Vec::new();
        for row in rows {
            auth_tokens.push(row?);
        }
        Ok(auth_tokens)
    }

    // Only one should be written at a time
    pub fn write_table(&self, auth_token: &AuthToken) -> Result<bool> {
        let result = self.connection.execute(
            "INSERT INTO authToken (date, token) VALUES (?1, ?2)",
            [
                auth_token.token.clone(),
                auth_token.time_interval.as_secs().to_string(),
            ],
        );
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                println!("Error WRITE to table: {}", e);
                Ok(false)
            }
        }
    }

    pub fn remove_auth_token(&self, auth_token: &AuthToken) -> Result<bool> {
        let result = self.connection.execute(
            "DELETE FROM authToken WHERE date = ?1 AND token = ?2",
            [
                auth_token.token.clone(),
                auth_token.time_interval.as_secs().to_string(),
            ],
        );
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                println!("Error DELETE to table: {}", e);
                Ok(false)
            }
        }
    }
}
