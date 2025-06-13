use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use uuid::Uuid;
use std::sync::Arc;
use crate::database::Database;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn create_session_cookie(user_id: Uuid) -> Cookie<'static> {
    Cookie::build(("session_id", user_id.to_string()))
        .path("/")
        .max_age(time::Duration::days(30))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build()
}

pub fn create_logout_cookie() -> Cookie<'static> {
    Cookie::build(("session_id", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build()
}

pub async fn auth_middleware(
    State(db): State<Arc<Database>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract session cookie
    let cookies = request.headers().get("cookie");
    
    if let Some(cookie_header) = cookies {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("session_id=") {
                    if let Ok(user_id) = cookie[11..].parse::<Uuid>() {
                        if let Ok(Some(_user)) = db.get_user_by_id(user_id).await {
                            request.extensions_mut().insert(user_id);
                            return Ok(next.run(request).await);
                        }
                    }
                }
            }
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}
