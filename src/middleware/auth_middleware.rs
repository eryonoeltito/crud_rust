use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tower_sessions::Session;

pub async fn wajib_login(session: Session, request: Request, next: Next) -> Response {
    let username = match session.get::<String>("username").await {
        Ok(username) => username,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Gagal memeriksa session"
                })),
            )
                .into_response();
        }
    };

    if username.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Silakan login terlebih dahulu"
            })),
        )
            .into_response();
    }

    next.run(request).await
}
