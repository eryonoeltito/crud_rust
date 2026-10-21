use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::barang::{Barang, BarangBaru, BarangUpdate};
use crate::views::barang_view::self;
use crate::views::format::format_nominal;

#[derive(Serialize)]
pub struct PesanResponse {
    pub message: String,
}

    pub async fn list_barang(
        State(pool): State<PgPool>,
    ) -> Result<Json<Vec<Vec<serde_json::Value>>>, (StatusCode, String)> {
        let barang = Barang::semua(&pool)
            .await
            .map_err(|error| {
                eprintln!("Gagal mengambil data barang: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal mengambil data barang".to_string(),
                )
            })?;

        Ok(barang_view::list_barang_response(barang))
    }

    pub async fn detail_barang(
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
    ) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
        let barang = Barang::berdasarkan_id(&pool, id)
            .await
            .map_err(|error| {
                eprintln!("Gagal mengambil detail barang: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal mengambil detail barang".to_string(),
                )
            })?;

        match barang {
            Some(item) => {
                let data = vec![
                    serde_json::json!(item.id),
                    serde_json::json!(item.nama),
                    serde_json::json!(item.stok),
                    serde_json::json!(format_nominal(item.harga)),
                ];

                Ok(Json(data))
            }
            None => Err((
                StatusCode::NOT_FOUND,
                format!("Barang dengan ID {id} tidak ditemukan"),
            )),
        }
    }


    pub async fn buat_barang(
        State(pool): State<PgPool>,
        Json(data): Json<BarangBaru>,
    ) -> Result<
                (StatusCode, Json<Vec<serde_json::Value>>),
                (StatusCode, String),
        > {
        
        if let Err(errors) = data.validasi() {
            return Err((
                StatusCode::BAD_REQUEST,
                errors.join("; "),
            ));
        }


        let barang = Barang::buat(&pool, data)
            .await
            .map_err(|error| {
                eprintln!("Gagal menambahkan barang: {error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal menambahkan barang".to_string(),
                )
            })?;

            Ok((
                StatusCode::CREATED,
                barang_view::detail_barang_response(barang),
            ))
    }


    pub async fn ubah_barang(
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
        Json(data): Json<BarangBaru>,
    ) -> Result<
                Json<Vec<serde_json::Value>>,
                (StatusCode, String),
        > {

        if let Err(errors) = data.validasi() {
            return Err((
                StatusCode::BAD_REQUEST,
                errors.join("; "),
            ));
        }

        let barang = Barang::ubah(&pool, id, data)
            .await
            .map_err(|error| {
                eprintln!("Gagal mengubah barang: {error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal mengubah barang".to_string(),
                )
            })?;

        match barang {
            Some(barang) => Ok(barang_view::detail_barang_response(barang)),
            None => Err((
                StatusCode::NOT_FOUND,
                format!("Barang dengan ID {id} tidak ditemukan"),
            )),
        }
    }

   
   
    pub async fn patch_barang(
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
        Json(data): Json<BarangUpdate>,
    ) -> Result<
                Json<Vec<serde_json::Value>>,
                (StatusCode, String),
        > {

        // Validasi hanya kolom yang dikirim pengguna.
        if data.nama.as_ref().is_some_and(|nama| nama.trim().is_empty()) {
            return Err((
                StatusCode::BAD_REQUEST,
                "Nama barang tidak boleh kosong".to_string(),
            ));
        }

        if data.stok.is_some_and(|stok| stok < 0) {
            return Err((
                StatusCode::BAD_REQUEST,
                "Stok barang tidak boleh negatif".to_string(),
            ));
        }

        if data.harga.is_some_and(|harga| harga < 0.0) {
            return Err((
                StatusCode::BAD_REQUEST,
                "Harga barang tidak boleh negatif".to_string(),
            ));
        }

        let barang = Barang::patch(&pool, id, data)
            .await
            .map_err(|error| {
                eprintln!("Gagal memperbarui sebagian data barang: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal memperbarui data barang".to_string(),
                )
            })?;

        match barang {
            Some(barang) => Ok(barang_view::detail_barang_response(barang)),
            None => Err((
                StatusCode::NOT_FOUND,
                format!("Barang dengan ID {id} tidak ditemukan"),
            )),
        }
    }


    pub async fn hapus_barang(
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
    ) -> Result<
        (StatusCode, Json<PesanResponse>),
        (StatusCode, String),
    > {
        let terhapus = Barang::hapus(&pool, id)
            .await
            .map_err(|error| {
                eprintln!("Gagal menghapus barang: {error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Gagal menghapus barang".to_string(),
                )
            })?;

        if terhapus {
            Ok((
                StatusCode::OK,
                Json(PesanResponse {
                    message: "Berhasil menghapus barang".to_string(),
                }),
            ))
        } else {
            Err((
                StatusCode::NOT_FOUND,
                format!("Barang dengan ID {id} tidak ditemukan"),
            ))
        }
    }


    