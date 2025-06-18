use axum::{
    
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
use crate::database::Database;

pub async fn analytics_middleware(
    State(db): State<Arc<Database>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    let path = request.uri().path().to_string();
    let method = request.method().to_string();
    
    // Extract session info
    let headers = request.headers();
    let _user_agent = headers.get("user-agent") // Prefix with underscore to avoid warning
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let referrer = headers.get("referer")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get or create session
    let session_id = get_or_create_session(&headers, &db).await;
    
    // Get user ID if authenticated
    let user_id = get_user_id_from_request(&headers, &db).await;

    // Add session ID to request extensions for handlers
    if let Some(session_uuid) = session_id {
        request.extensions_mut().insert(session_uuid);
    }

    // Process the request
    let response = next.run(request).await;
    let status_code = response.status().as_u16() as i32;
    let response_time = start_time.elapsed().as_millis() as i32;

    // Log the page view (don't block the response)
    if let Some(session_uuid) = session_id {
        let db_clone = db.clone();
        tokio::spawn(async move {
            let _ = db_clone.log_page_view(LogPageViewRequest {
                session_id: session_uuid,
                user_id,
                path,
                method,
                status_code: Some(status_code),
                response_time_ms: Some(response_time),
                referrer,
            }).await;
        });
    }

    Ok(response)
}

async fn get_or_create_session(headers: &HeaderMap, db: &Database) -> Option<Uuid> {
    // Try to get session from cookie
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("analytics_session=") {
                    let session_id = &cookie[18..];
                    if let Ok(uuid) = session_id.parse::<Uuid>() {
                        // Check if session exists in database
                        if let Ok(Some(_)) = db.get_session_by_uuid(uuid).await {
                            return Some(uuid);
                        }
                    }
                }
            }
        }
    }

    // Create new session
    let new_session_id = Uuid::new_v4();
    let user_agent = headers.get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    if let Ok(_) = db.create_user_session(CreateSessionRequest {
        session_id: new_session_id.to_string(),
        user_id: None,
        ip_address: None,
        user_agent,
    }).await {
        Some(new_session_id)
    } else {
        None
    }
}

async fn get_user_id_from_request(headers: &HeaderMap, db: &Database) -> Option<Uuid> {
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("session_id=") {
                    if let Ok(user_id) = cookie[11..].parse::<Uuid>() {
                        if let Ok(Some(_)) = db.get_user_by_id(user_id).await {
                            return Some(user_id);
                        }
                    }
                }
            }
        }
    }
    None
}

pub struct CreateSessionRequest {
    pub session_id: String,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
}

pub struct LogPageViewRequest {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub path: String,
    pub method: String,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub referrer: Option<String>,
}

pub struct LogEventRequest {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub event_data: Option<serde_json::Value>,
}
