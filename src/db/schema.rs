use rusqlite::{Connection, Result as SqlResult};

pub fn initialize_schema(conn: &Connection) -> SqlResult<()> {
    // Create categories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            icon TEXT
        )",
        [],
    )?;

    // Create transactions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            amount REAL NOT NULL,
            description TEXT,
            date TEXT NOT NULL,
            type TEXT NOT NULL,
            category_id TEXT,
            FOREIGN KEY (category_id) REFERENCES categories (id)
        )",
        [],
    )?;

    // Create budgets table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS budgets (
            id TEXT PRIMARY KEY,
            category_id TEXT,
            amount REAL NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories (id)
        )",
        [],
    )?;
    
    // Initialize default categories if they don't exist
    initialize_default_categories(conn)?;
    
    Ok(())
}

fn initialize_default_categories(conn: &Connection) -> SqlResult<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))?;
    
    if count == 0 {
        let default_categories = [
            ("cat_1", "Housing", "#4C8BF5"),
            ("cat_2", "Food", "#2D9D5A"),
            ("cat_3", "Transportation", "#FF9900"),
            ("cat_4", "Entertainment", "#DB4437"),
            ("cat_5", "Healthcare", "#673AB7"),
            ("cat_6", "Utilities", "#795548"),
            ("cat_7", "Shopping", "#E91E63"),
            ("cat_8", "Personal", "#607D8B"),
            ("cat_9", "Income", "#0F9D58"),
            ("cat_10", "Other", "#9E9E9E"),
        ];
        
        let tx = conn.transaction()?;
        
        for (id, name, color) in default_categories.iter() {
            tx.execute(
                "INSERT INTO categories (id, name, color) VALUES (?1, ?2, ?3)",
                [id, name, color],
            )?;
        }
        
        tx.commit()?;
    }
    
    Ok(())
}
