use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{Utc, Duration};
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::models::*;
use crate::analytics::*;

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

    // Analytics methods
    pub async fn create_user_session(&self, req: CreateSessionRequest) -> Result<UserSession> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let session = sqlx::query_as::<_, UserSession>(
            "INSERT INTO user_sessions (id, user_id, session_id, ip_address, user_agent, started_at, last_activity)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(id)
        .bind(req.user_id)
        .bind(req.session_id)
        .bind(req.ip_address.map(|ip| ip.to_string())) // Convert IpAddr to String
        .bind(req.user_agent)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(session)
    }

    pub async fn log_page_view(&self, req: LogPageViewRequest) -> Result<()> {
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO page_views (id, session_id, user_id, path, method, status_code, response_time_ms, referrer)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(id)
        .bind(req.session_id)
        .bind(req.user_id)
        .bind(req.path)
        .bind(req.method)
        .bind(req.status_code)
        .bind(req.response_time_ms)
        .bind(req.referrer)
        .execute(&self.pool)
        .await?;

        // Update session last_activity
        sqlx::query(
            "UPDATE user_sessions SET last_activity = NOW() WHERE id = $1"
        )
        .bind(req.session_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn log_user_event(&self, req: LogEventRequest) -> Result<()> {
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO user_events (id, session_id, user_id, event_type, event_data)
             VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(id)
        .bind(req.session_id)
        .bind(req.user_id)
        .bind(req.event_type)
        .bind(req.event_data)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_analytics_dashboard(&self) -> Result<AnalyticsDashboard> {
        let now = Utc::now();
        let today = now.date_naive();
        let week_ago = now - Duration::days(7);

        // Total users
        let total_users: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users"
        ).fetch_one(&self.pool).await?;

        // Active sessions (last 30 minutes)
        let active_sessions: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM user_sessions WHERE last_activity > NOW() - INTERVAL '30 minutes'"
        ).fetch_one(&self.pool).await?;

        // Page views today
        let page_views_today: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM page_views WHERE viewed_at::date = $1"
        ).bind(today).fetch_one(&self.pool).await?;

        // Popular pages (last 7 days)
        let popular_pages = sqlx::query_as::<_, PopularPage>(
            "SELECT path, COUNT(*) as view_count, COUNT(DISTINCT user_id) as unique_users
             FROM page_views 
             WHERE viewed_at > $1 AND path NOT LIKE '/api%'
             GROUP BY path 
             ORDER BY view_count DESC 
             LIMIT 10"
        ).bind(week_ago).fetch_all(&self.pool).await?;

        // User activity summary
        let user_activity = sqlx::query_as::<_, UserActivitySummary>(
            "SELECT 
                s.user_id,
                u.full_name as user_name,
                u.email as user_email,
                MAX(s.last_activity) as last_activity,
                COUNT(DISTINCT s.id) as session_count,
                COUNT(pv.id) as page_views,
                COALESCE(SUM(s.duration_seconds), 0) / 60 as total_time_minutes
             FROM user_sessions s
             LEFT JOIN users u ON s.user_id = u.id
             LEFT JOIN page_views pv ON s.id = pv.session_id
             WHERE s.started_at > $1
             GROUP BY s.user_id, u.full_name, u.email
             ORDER BY last_activity DESC
             LIMIT 20"
        ).bind(week_ago).fetch_all(&self.pool).await?;

        // Daily stats (last 7 days)
        let daily_stats = sqlx::query_as::<_, DailyStats>(
            "SELECT 
                date_trunc('day', viewed_at)::date as date,
                COUNT(DISTINCT user_id) as unique_users,
                COUNT(*) as page_views,
                0 as new_registrations,
                0 as active_sessions
             FROM page_views 
             WHERE viewed_at > $1
             GROUP BY date_trunc('day', viewed_at)::date
             ORDER BY date DESC"
        ).bind(week_ago).fetch_all(&self.pool).await?;

        // Recent events
        let recent_events = sqlx::query_as::<_, UserEvent>(
            "SELECT * FROM user_events 
             ORDER BY created_at DESC 
             LIMIT 50"
        ).fetch_all(&self.pool).await?;

        Ok(AnalyticsDashboard {
            total_users: total_users.0,
            active_sessions: active_sessions.0,
            page_views_today: page_views_today.0,
            popular_pages,
            user_activity,
            daily_stats,
            recent_events,
        })
    }

    pub async fn get_session_by_uuid(&self, session_id: Uuid) -> Result<Option<UserSession>> {
        let session = sqlx::query_as::<_, UserSession>(
            "SELECT * FROM user_sessions WHERE id = $1"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(session)
    }

    // Add public getter for pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
