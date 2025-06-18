use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    Extension,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::database::Database;
use crate::models::*;

pub async fn analytics_dashboard() -> Html<&'static str> {
    // Return a simple HTML page that redirects to the static file
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <title>Analytics Dashboard</title>
</head>
<body>
    <script>window.location.href = '/static/analytics.html';</script>
</body>
</html>"#)
}

pub async fn get_analytics_data(
    State(db): State<Arc<Database>>,
    Extension(_user_id): Extension<Uuid>,
) -> Result<Json<AnalyticsDashboard>, StatusCode> {
    let dashboard = db.get_analytics_dashboard()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(dashboard))
}

pub async fn get_user_sessions(
    State(db): State<Arc<Database>>,
    Extension(_user_id): Extension<Uuid>,
) -> Result<Json<Vec<UserSession>>, StatusCode> {
    let sessions = sqlx::query_as::<_, UserSession>(
        "SELECT * FROM user_sessions 
         ORDER BY started_at DESC 
         LIMIT 100"
    )
    .fetch_all(db.pool()) // Use the public getter method
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sessions))
}
