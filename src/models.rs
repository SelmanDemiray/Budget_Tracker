use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::net::IpAddr;

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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: String,
    pub ip_address: Option<String>, // Changed from IpAddr to String
    pub user_agent: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub duration_seconds: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PageView {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub path: String,
    pub method: String,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub referrer: Option<String>,
    pub viewed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub event_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsDashboard {
    pub total_users: i64,
    pub active_sessions: i64,
    pub page_views_today: i64,
    pub popular_pages: Vec<PopularPage>,
    pub user_activity: Vec<UserActivitySummary>,
    pub daily_stats: Vec<DailyStats>,
    pub recent_events: Vec<UserEvent>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PopularPage {
    pub path: String,
    pub view_count: i64,
    pub unique_users: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserActivitySummary {
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub last_activity: DateTime<Utc>,
    pub session_count: i64,
    pub page_views: i64,
    pub total_time_minutes: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DailyStats {
    pub date: chrono::NaiveDate,
    pub unique_users: i64,
    pub page_views: i64,
    pub new_registrations: i64,
    pub active_sessions: i64,
}
