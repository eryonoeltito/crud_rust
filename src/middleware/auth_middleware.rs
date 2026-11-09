use std::sync::Arc;

use crate::views::error_view::error_response;
use crate::{
    auth::validasi_token,
    models::user::{AkunTerverifikasi, User},
};
use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

pub async fn wajib_login(
    Extension(jwt_secret): Extension<Arc<str>>,
    Extension(pool): Extension<PgPool>,
    mut request: Request,
    next: Next,
) -> Response {
    let Some(token) = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|nilai| nilai.to_str().ok())
        .and_then(|nilai| nilai.strip_prefix("Bearer "))
    else {
        return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
    };

    let Ok(klaim) = validasi_token(token, &jwt_secret) else {
        return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
    };

    let id = match klaim.sub.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
        }
    };

    let mut koneksi = match pool.acquire().await {
        Ok(koneksi) => koneksi,
        Err(kesalahan) => {
            eprintln!("Gagal membuka koneksi untuk autentikasi: {kesalahan}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal memeriksa autentikasi",
            );
        }
    };

    let akun = match User::ambil_akun_terverifikasi(&mut koneksi, id).await {
        Ok(Some(akun)) => akun,
        Ok(None) => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "Akun tidak ditemukan atau tidak aktif",
            );
        }
        Err(kesalahan) => {
            eprintln!("Gagal mengambil akun untuk autentikasi: {kesalahan}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal memeriksa autentikasi",
            );
        }
    };

    request.extensions_mut().insert::<AkunTerverifikasi>(akun);
    next.run(request).await
}

