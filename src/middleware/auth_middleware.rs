use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::auth::validasi_token;

pub async fn wajib_login(
    Extension(jwt_secret): Extension<Arc<str>>,
    request: Request,
    next: Next,
) -> Response {
    let token_valid = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| validasi_token(token, &jwt_secret).is_ok());

    if !token_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Token tidak ada atau tidak valid"
            })),
        )
            .into_response();
    }

    next.run(request).await
}
