use std::sync::Arc;

use crate::views::error_view::error_response;
use crate::{
    auth::validasi_token,
    models::{
        revoked_token::periksa_token_dicabut,
        user::{AkunTerverifikasi, User},
    },
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

    match periksa_token_dicabut(&pool, &klaim.jti).await {
        Ok(true) => {
            return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
        }
        Ok(false) => {}
        Err(kesalahan) => {
            eprintln!("Gagal memeriksa pencabutan token: {kesalahan}");
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal memeriksa autentikasi",
            );
        }
    }

    let id = match klaim.sub.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return error_response(StatusCode::UNAUTHORIZED, "Token tidak ada atau tidak valid");
        }
    };

    let akun = match User::ambil_akun_terverifikasi(&pool, id).await {
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

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, header::AUTHORIZATION};

    fn ambil_token_bearer(header: &HeaderMap) -> Option<&str> {
        header
            .get(AUTHORIZATION)
            .and_then(|nilai| nilai.to_str().ok())
            .and_then(|nilai| nilai.strip_prefix("Bearer "))
    }

    #[test]
    fn mengambil_token_bearer_dengan_format_benar() {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, "Bearer token-uji".parse().unwrap());

        assert_eq!(ambil_token_bearer(&header), Some("token-uji"));
    }

    #[test]
    fn menolak_header_authorization_yang_tidak_valid() {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, "Basic token-uji".parse().unwrap());

        assert_eq!(ambil_token_bearer(&header), None);
    }
}
