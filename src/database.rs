use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::Utc;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::models::*;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn create_user(&self, email: &str, password_hash: &str, full_name: &str) -> Result<User> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, password_hash, full_name, created_at) 
             VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(full_name)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM budget_entries WHERE user_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn save_budget_entry(&self, user_id: Uuid, entry: &BudgetUpdateRequest) -> Result<()> {
        let amount = BigDecimal::from_str(&entry.amount.to_string()).unwrap_or_default();
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query(
            "INSERT INTO budget_entries (id, user_id, category, subcategory, month, year, amount, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (user_id, category, subcategory, month, year)
             DO UPDATE SET amount = $7, updated_at = $9"
        )
        .bind(id)
        .bind(user_id)
        .bind(&entry.category)
        .bind(&entry.subcategory)
        .bind(entry.month)
        .bind(entry.year)
        .bind(amount)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_budget_entries(&self, user_id: Uuid, year: i32) -> Result<Vec<BudgetEntry>> {
        let entries = sqlx::query_as::<_, BudgetEntry>(
            "SELECT * FROM budget_entries WHERE user_id = $1 AND year = $2 ORDER BY category, subcategory, month"
        )
        .bind(user_id)
        .bind(year)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(entries)
    }
}
