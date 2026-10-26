use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

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
) -> axum::response::Response {
    let valid = match User::cek_login(&pool, &input.username, &input.password).await {
        Ok(valid) => valid,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Terjadi kesalahan pada server"
                })),
            )
                .into_response();
        }
    };

    if !valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Username atau password salah"
            })),
        )
            .into_response();
    }

    if session.insert("username", input.username).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Gagal membuat session"
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Login berhasil",
            "session": "aktif"
        })),
    )
        .into_response()
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
