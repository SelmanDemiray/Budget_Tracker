use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    Extension,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::Datelike;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::{create_session_cookie, create_logout_cookie, hash_password, verify_password},
    budget::get_default_categories,
    database::Database,
    models::*,
};

pub async fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Budget Tracker</title>
    <link rel="stylesheet" href="/static/styles.css">
</head>
<body>
    <script>window.location.href = '/static/index.html';</script>
</body>
</html>"#)
}

pub async fn register(
    State(db): State<Arc<Database>>,
    jar: CookieJar,
    Json(payload): Json<RegisterRequest>,
) -> Result<(CookieJar, Json<UserPublic>), StatusCode> {
    // Validate input
    if payload.email.is_empty() || payload.password.len() < 6 || payload.full_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    if db.get_user_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash = hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user
    let user = db.create_user(&payload.email, &password_hash, &payload.full_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_public = UserPublic {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        created_at: user.created_at,
    };

    let jar = jar.add(create_session_cookie(user.id));
    Ok((jar, Json(user_public)))
}

pub async fn login(
    State(db): State<Arc<Database>>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, Json<UserPublic>), StatusCode> {
    let user = db.get_user_by_email(&payload.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !verify_password(&payload.password, &user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_public = UserPublic {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        created_at: user.created_at,
    };

    let jar = jar.add(create_session_cookie(user.id));
    Ok((jar, Json(user_public)))
}

pub async fn logout(jar: CookieJar) -> CookieJar {
    jar.add(create_logout_cookie())
}

pub async fn get_user(
    State(db): State<Arc<Database>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserPublic>, StatusCode> {
    let user = db.get_user_by_id(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user_public = UserPublic {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        created_at: user.created_at,
    };

    Ok(Json(user_public))
}

pub async fn delete_user(
    State(db): State<Arc<Database>>,
    jar: CookieJar,
    Extension(user_id): Extension<Uuid>,
) -> Result<CookieJar, StatusCode> {
    db.delete_user(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(jar.add(create_logout_cookie()))
}

#[derive(Deserialize)]
pub struct YearQuery {
    year: Option<i32>,
}

pub async fn get_budget(
    State(db): State<Arc<Database>>,
    Extension(user_id): Extension<Uuid>,
    Query(params): Query<YearQuery>,
) -> Result<Json<Vec<BudgetEntry>>, StatusCode> {
    let year = params.year.unwrap_or_else(|| chrono::Utc::now().year());
    
    let entries = db.get_budget_entries(user_id, year)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(entries))
}

pub async fn save_budget(
    State(db): State<Arc<Database>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<BudgetUpdateRequest>,
) -> Result<StatusCode, StatusCode> {
    db.save_budget_entry(user_id, &payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn get_categories() -> Json<Vec<BudgetCategory>> {
    Json(get_default_categories())
}
