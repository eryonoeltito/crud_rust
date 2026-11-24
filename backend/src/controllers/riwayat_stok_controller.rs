use axum::{
    Json,
    extract::{Extension, Path, State, rejection::PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

use crate::{
    models::{riwayat_stok::RiwayatStok, user::AkunTerverifikasi},
    views::error_view::error_response,
};

pub async fn list_riwayat_stok(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
) -> Result<Json<Vec<RiwayatStok>>, Response> {
    let hasil = match akun.role.as_deref() {
        Some("admin") => RiwayatStok::semua(&pool).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => RiwayatStok::di_gudang(&pool, gudang_id).await,
            None => {
                return Err(error_response(
                    StatusCode::FORBIDDEN,
                    "Akun petugas tidak memiliki gudang yang ditugaskan",
                ));
            }
        },
        _ => {
            return Err(error_response(
                StatusCode::FORBIDDEN,
                "Akun tidak memiliki hak untuk melihat riwayat stok",
            ));
        }
    };

    let riwayat = hasil.map_err(|kesalahan| {
        eprintln!("Gagal mengambil riwayat stok: {kesalahan}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil riwayat stok",
        )
    })?;

    Ok(Json(riwayat))
}

pub async fn list_riwayat_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    path: Result<Path<i32>, PathRejection>,
) -> Result<Json<Vec<RiwayatStok>>, Response> {
    let Path(barang_id) = path.map_err(|penolakan| {
        error_response(
            penolakan.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;

    let (ada, gudang_id) = match akun.role.as_deref() {
        Some("admin") => (RiwayatStok::barang_ada(&pool, barang_id).await, None),
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => (
                RiwayatStok::barang_ada_di_gudang(&pool, barang_id, gudang_id).await,
                Some(gudang_id),
            ),
            None => {
                return Err(error_response(
                    StatusCode::FORBIDDEN,
                    "Akun petugas tidak memiliki gudang yang ditugaskan",
                ));
            }
        },
        _ => {
            return Err(error_response(
                StatusCode::FORBIDDEN,
                "Akun tidak memiliki hak untuk melihat riwayat stok",
            ));
        }
    };

    let ada = ada.map_err(|kesalahan| {
        eprintln!("Gagal memeriksa barang untuk riwayat stok: {kesalahan}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil riwayat stok",
        )
    })?;

    if !ada {
        return Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {barang_id} tidak ditemukan"),
        ));
    }

    let riwayat = match gudang_id {
        Some(gudang_id) => {
            RiwayatStok::berdasarkan_barang_di_gudang(&pool, barang_id, gudang_id).await
        }
        None => RiwayatStok::berdasarkan_barang(&pool, barang_id).await,
    }
    .map_err(|kesalahan| {
        eprintln!("Gagal mengambil riwayat barang: {kesalahan}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil riwayat stok",
        )
    })?;

    Ok(Json(riwayat))
}
