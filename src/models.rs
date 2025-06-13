use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BudgetEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub subcategory: String,
    pub month: i32,
    pub year: i32,
    pub amount: bigdecimal::BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BudgetUpdateRequest {
    pub category: String,
    pub subcategory: String,
    pub month: i32,
    pub year: i32,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct BudgetCategory {
    pub name: String,
    pub subcategories: Vec<String>,
    pub is_income: bool,
}
