use axum::{
    middleware,
    routing::{get, post, delete},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use std::sync::Arc;

mod auth;
mod budget;
mod database;
mod handlers;
mod models;
mod analytics;
mod analytics_handlers;

use database::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    
    // Initialize logging
    env_logger::init();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/budget_tracker".to_string());
    
    let db = Database::new(&database_url).await?;
    db.migrate().await?;
    
    let app_state = Arc::new(db);
    
    // Configure CORS for production vs development
    let cors = if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        CorsLayer::new()
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::DELETE])
            .allow_headers([axum::http::header::CONTENT_TYPE])
            .allow_credentials(true)
    } else {
        CorsLayer::permissive()
    };
    
    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/api/user", get(handlers::get_user))
        .route("/api/user", delete(handlers::delete_user))
        .route("/api/budget", get(handlers::get_budget))
        .route("/api/budget", post(handlers::save_budget))
        .route("/api/analytics/dashboard", get(analytics_handlers::get_analytics_data))
        .route("/api/analytics/sessions", get(analytics_handlers::get_user_sessions))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::auth_middleware,
        ));

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/analytics", get(analytics_handlers::analytics_dashboard))
        .route("/api/register", post(handlers::register))
        .route("/api/login", post(handlers::login))
        .route("/api/logout", post(handlers::logout))
        .route("/api/budget/categories", get(handlers::get_categories))
        .merge(protected_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(cors)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            analytics::analytics_middleware,
        ))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    println!("Analytics dashboard: http://0.0.0.0:3000/analytics");
    
    axum::serve(listener, app).await?;
    Ok(())
}
