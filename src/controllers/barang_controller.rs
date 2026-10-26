use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::barang::{Barang, BarangBaru, BarangUpdate};
use crate::views::barang_view;

#[derive(Serialize)]
pub struct PesanResponse {
    pub message: String,
}

pub async fn list_barang(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<barang_view::BarangResponse>>, (StatusCode, String)> {
    let barang = Barang::semua(&pool).await.map_err(|error| {
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
) -> Result<Json<barang_view::BarangResponse>, (StatusCode, String)> {
    let barang = Barang::berdasarkan_id(&pool, id).await.map_err(|error| {
        eprintln!("Gagal mengambil detail barang: {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil detail barang".to_string(),
        )
    })?;

    match barang {
        Some(item) => Ok(barang_view::detail_barang_response(item)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn buat_barang(
    State(pool): State<PgPool>,
    Json(data): Json<BarangBaru>,
) -> Result<(StatusCode, Json<barang_view::BarangResponse>), (StatusCode, String)> {
    if let Err(errors) = data.validasi() {
        return Err((StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let barang = Barang::buat(&pool, data).await.map_err(|error| {
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
) -> Result<Json<barang_view::BarangResponse>, (StatusCode, String)> {
    if let Err(errors) = data.validasi() {
        return Err((StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let barang = Barang::ubah(&pool, id, data).await.map_err(|error| {
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
) -> Result<Json<barang_view::BarangResponse>, (StatusCode, String)> {
    if data.nama.is_none() && data.stok.is_none() && data.harga.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "PATCH harus menyertakan minimal satu field untuk diperbarui".to_string(),
        ));
    }

    // Validasi hanya kolom yang dikirim pengguna.
    if data
        .nama
        .as_ref()
        .is_some_and(|nama| nama.trim().is_empty())
    {
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

    if data
        .harga
        .is_some_and(|harga| harga < rust_decimal::Decimal::ZERO)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Harga barang tidak boleh negatif".to_string(),
        ));
    }

    let barang = Barang::patch(&pool, id, data).await.map_err(|error| {
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
) -> Result<(StatusCode, Json<PesanResponse>), (StatusCode, String)> {
    let terhapus = Barang::hapus(&pool, id).await.map_err(|error| {
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
