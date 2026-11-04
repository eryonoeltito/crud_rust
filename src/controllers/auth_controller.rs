use std::sync::Arc;

use axum::extract::{Extension, rejection::JsonRejection};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
};

use crate::auth::{buat_token, validasi_token};
use crate::models::revoked_token::cabut_token;
use crate::models::user::User;
use crate::views::error_view::error_response;
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
    body: Result<Json<RegisterRequest>, JsonRejection>,
) -> axum::response::Response {
    let Json(input) = match body {
        Ok(body) => body,
        Err(rejection) => {
            return error_response(rejection.into_response().status(), "Body JSON tidak valid");
        }
    };
    if input.username.trim().is_empty() || input.password.is_empty() {
        return error_response(StatusCode::BAD_REQUEST, "Username dan password wajib diisi");
    }

    let registered = match User::register(&pool, &input.username, &input.password).await {
        Ok(registered) => registered,
        Err(error) => {
            eprintln!("Gagal mendaftarkan pengguna: {error}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada server",
            );
        }
    };

    if !registered {
        return error_response(StatusCode::CONFLICT, "Username sudah digunakan");
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
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> axum::response::Response {
    let Json(input) = match body {
        Ok(body) => body,
        Err(rejection) => {
            return error_response(rejection.into_response().status(), "Body JSON tidak valid");
        }
    };
    let mut koneksi = match pool.acquire().await {
        Ok(koneksi) => koneksi,
        Err(error) => {
            eprintln!("Gagal membuka koneksi untuk login: {error}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada server",
            );
        }
    };

    let user_id = match User::cek_login(&mut koneksi, &input.username, &input.password).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => {
            return error_response(StatusCode::UNAUTHORIZED, "Username atau password salah");
        }
        Err(error) => {
            eprintln!("Gagal memeriksa login: {error}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada server",
            );
        }
    };

    let token = match buat_token(user_id, &jwt_secret) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Gagal membuat token autentikasi: {error:?}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal membuat token autentikasi",
            );
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

pub async fn logout(
    State(pool): State<PgPool>,
    Extension(jwt_secret): Extension<Arc<str>>,
    headers: HeaderMap,
) -> Response {
    let Some(token) = headers
        .get(AUTHORIZATION)
        .and_then(|nilai| nilai.to_str().ok())
        .and_then(|nilai| nilai.strip_prefix("Bearer "))
    else {
        return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
    };

    let klaim = match validasi_token(token, &jwt_secret) {
        Ok(klaim) => klaim,
        Err(_) => {
            return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
        }
    };

    let Ok(waktu_kedaluwarsa) = i64::try_from(klaim.exp) else {
        return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
    };

    let mut koneksi = match pool.acquire().await {
        Ok(koneksi) => koneksi,
        Err(error) => {
            eprintln!("Gagal membuka koneksi untuk logout: {error}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kesalahan pada server",
            );
        }
    };

    if let Err(error) = cabut_token(&mut koneksi, &klaim.jti, waktu_kedaluwarsa).await {
        eprintln!("Gagal mencabut token: {error}");
        return error_response(StatusCode::INTERNAL_SERVER_ERROR, "Gagal mencabut token");
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Logout berhasil"
        })),
    )
        .into_response()
}
