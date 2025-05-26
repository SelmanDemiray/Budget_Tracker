pub mod operations;
pub mod schema;

use rusqlite::{Connection, Result};
use std::path::Path;

pub struct DatabaseConnection {
    connection: Connection,
}

impl DatabaseConnection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(DatabaseConnection { connection })
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    
    pub fn initialize_schema(&self) -> Result<()> {
        // Create transactions table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                amount REAL NOT NULL,
                description TEXT,
                date TEXT NOT NULL,
                category_id TEXT,
                FOREIGN KEY (category_id) REFERENCES categories (id)
            )",
            [],
        )?;
        
        // Create categories table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT NOT NULL
            )",
            [],
        )?;
        
        // Create budgets table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS budgets (
                id TEXT PRIMARY KEY,
                category_id TEXT NOT NULL,
                amount REAL NOT NULL,
                period TEXT NOT NULL,
                FOREIGN KEY (category_id) REFERENCES categories (id)
            )",
            [],
        )?;
        
        Ok(())
    }
}
