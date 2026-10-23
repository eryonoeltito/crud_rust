
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_sessions::Session;

pub async fn wajib_login(
    session: Session,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let username = session
        .get::<String>("username")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match username {
        Some(_) => Ok(next.run(request).await),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
