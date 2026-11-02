use std::sync::Arc;

use axum::extract::Extension;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::auth::buat_token;
use crate::models::user::User;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(input): Json<RegisterRequest>,
) -> axum::response::Response {
    if input.username.trim().is_empty() || input.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Username dan password wajib diisi"
            })),
        )
            .into_response();
    }

    let registered = match User::register(&pool, &input.username, &input.password).await {
        Ok(registered) => registered,
        Err(error) => {
            eprintln!("Gagal mendaftarkan pengguna: {error}");
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

    if !registered {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "status": "error",
                "message": "Username sudah digunakan"
            })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "Pendaftaran berhasil"
        })),
    )
        .into_response()
}

pub async fn login(
    State(pool): State<PgPool>,
    Extension(jwt_secret): Extension<Arc<str>>,
    Json(input): Json<LoginRequest>,
) -> axum::response::Response {
    let valid = match User::cek_login(&pool, &input.username, &input.password).await {
        Ok(valid) => valid,
        Err(error) => {
            eprintln!("Gagal memeriksa login: {error}");
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

    let token = match buat_token(&input.username, &jwt_secret) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat token autentikasi: {error:?}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Gagal membuat token autentikasi"
                })),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Login berhasil",
            "token_type": "Bearer",
            "token": token,
            "expires_in": 3600
        })),
    )
        .into_response()
}

pub async fn logout() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message": "Logout berhasil. Hapus token dari client."
    }))
}
