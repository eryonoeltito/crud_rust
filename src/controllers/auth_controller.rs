
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::models::user::User;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(input): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let valid = User::cek_login(
        &pool,
        &input.username,
        &input.password,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    session
        .insert("username", input.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": "Login berhasil",
        "session": "aktif"
    })))
}


pub async fn logout(
    session: tower_sessions::Session,
) -> Result<Json<serde_json::Value>, StatusCode> {
    session
        .flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": "Logout berhasil"
    })))
}

