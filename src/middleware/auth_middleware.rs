use std::sync::Arc;

use crate::auth::validasi_token;
use crate::views::error_view::error_response;
use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

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
        return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
    }

    next.run(request).await
}
