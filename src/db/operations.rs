use crate::models::{Budget, Category, Transaction, TransactionType};
use chrono::NaiveDate;
use rusqlite::{params, Connection, Result as SqlResult};

// Transaction operations
pub fn add_transaction(conn: &Connection, transaction: &Transaction) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO transactions (id, amount, description, date, category_id, transaction_type)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            transaction.id,
            transaction.amount,
            transaction.description,
            transaction.date.to_string(),
            transaction.category_id,
            transaction.transaction_type.to_string(),
        ],
    )?;
    
    Ok(())
}

pub fn update_transaction(conn: &Connection, transaction: &Transaction) -> SqlResult<()> {
    conn.execute(
        "UPDATE transactions
         SET amount = ?1, description = ?2, date = ?3, category_id = ?4, transaction_type = ?5
         WHERE id = ?6",
        params![
            transaction.amount,
            transaction.description,
            transaction.date.to_string(),
            transaction.category_id,
            transaction.transaction_type.to_string(),
            transaction.id,
        ],
    )?;
    
    Ok(())
}

pub fn delete_transaction(conn: &Connection, transaction_id: &str) -> SqlResult<()> {
    conn.execute(
        "DELETE FROM transactions WHERE id = ?1",
        [transaction_id],
    )?;
    
    Ok(())
}

pub fn get_all_transactions(conn: &Connection) -> SqlResult<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, amount, description, date, category_id, transaction_type
         FROM transactions
         ORDER BY date DESC"
    )?;
    
    let transaction_iter = stmt.query_map([], |row| {
        let date_str: String = row.get(3)?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Local::now().naive_local().date());
        
        let transaction_type_str: String = row.get(5)?;
        let transaction_type = match transaction_type_str.as_str() {
            "Income" => TransactionType::Income,
            _ => TransactionType::Expense,
        };
        
        Ok(Transaction {
            id: row.get(0)?,
            amount: row.get(1)?,
            description: row.get(2)?,
            date,
            category_id: row.get(4)?,
            transaction_type,
        })
    })?;
    
    let mut transactions = Vec::new();
    for transaction in transaction_iter {
        transactions.push(transaction?);
    }
    
    Ok(transactions)
}

// Category operations
pub fn add_category(conn: &Connection, category: &Category) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO categories (id, name, color, icon)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            category.id,
            category.name,
            category.color,
            category.icon,
        ],
    )?;
    
    Ok(())
}

pub fn get_all_categories(conn: &Connection) -> SqlResult<Vec<Category>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, color, icon
         FROM categories
         ORDER BY name"
    )?;
    
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            icon: row.get::<_, Option<String>>(3)?,
        })
    })?;
    
    let mut categories = Vec::new();
    for category in category_iter {
        categories.push(category?);
    }
    
    Ok(categories)
}

// Budget operations
pub fn add_budget(conn: &Connection, budget: &Budget) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO budgets (id, category_id, amount, start_date, end_date, name)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            budget.id,
            budget.category_id,
            budget.amount,
            budget.start_date.to_string(),
            budget.end_date.to_string(),
            budget.name,
        ],
    )?;
    
    Ok(())
}

pub fn update_budget(conn: &Connection, budget: &Budget) -> SqlResult<()> {
    conn.execute(
        "UPDATE budgets
         SET category_id = ?1, amount = ?2, start_date = ?3, end_date = ?4, name = ?5
         WHERE id = ?6",
        params![
            budget.category_id,
            budget.amount,
            budget.start_date.to_string(),
            budget.end_date.to_string(),
            budget.name,
            budget.id,
        ],
    )?;
    
    Ok(())
}

pub fn delete_budget(conn: &Connection, budget_id: &str) -> SqlResult<()> {
    conn.execute(
        "DELETE FROM budgets WHERE id = ?1",
        [budget_id],
    )?;
    
    Ok(())
}

pub fn get_all_budgets(conn: &Connection) -> SqlResult<Vec<Budget>> {
    let mut stmt = conn.prepare(
        "SELECT id, category_id, amount, start_date, end_date, name
         FROM budgets
         ORDER BY start_date DESC"
    )?;
    
    let budget_iter = stmt.query_map([], |row| {
        let start_date_str: String = row.get(3)?;
        let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Local::now().naive_local().date());
            
        let end_date_str: String = row.get(4)?;
        let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Local::now().naive_local().date());
        
        Ok(Budget {
            id: row.get(0)?,
            category_id: row.get(1)?,
            amount: row.get(2)?,
            start_date,
            end_date,
            name: row.get(5)?,
        })
    })?;
    
    let mut budgets = Vec::new();
    for budget in budget_iter {
        budgets.push(budget?);
    }
    
    Ok(budgets)
}
