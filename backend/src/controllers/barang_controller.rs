use axum::{
    Json,
    extract::{
        Extension, Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::{
    barang::{Barang, BarangBaru, BarangUpdate, PermintaanBarangBaru},
    user::AkunTerverifikasi,
};
use crate::views::{barang_view, error_view::error_response};

#[derive(Serialize)]
pub struct PesanResponse {
    pub message: String,
}

pub async fn list_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
) -> Result<Json<Vec<barang_view::BarangResponse>>, Response> {
    let hasil = match akun.role.as_deref() {
        Some("admin") => Barang::semua(&pool).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => Barang::di_gudang(&pool, gudang_id).await,
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
                "Akun tidak memiliki hak untuk melihat barang",
            ));
        }
    };
    
    let barang = hasil.map_err(|error| {
        eprintln!("Gagal mengambil data barang: {error}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil data barang",
        )
    })?;

    Ok(barang_view::list_barang_response(barang))
}

pub async fn detail_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    path: Result<Path<i32>, PathRejection>,
) -> Result<Json<barang_view::BarangResponse>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let hasil = match akun.role.as_deref() {
        Some("admin") => Barang::berdasarkan_id(&pool, id).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => Barang::berdasarkan_id_di_gudang(&pool, id, gudang_id).await,
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
                "Akun tidak memiliki hak untuk melihat barang",
            ));
        }
    };

    let barang = hasil.map_err(|error| {
        eprintln!("Gagal mengambil detail barang: {error}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil detail barang",
        )
    })?;

    match barang {
        Some(item) => Ok(barang_view::detail_barang_response(item)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn buat_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    body: Result<Json<PermintaanBarangBaru>, JsonRejection>,
) -> Result<(StatusCode, Json<barang_view::BarangResponse>), Response> {
    let Json(permintaan) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    let data = BarangBaru {
        nama: permintaan.nama,
        stok: permintaan.stok,
        harga: permintaan.harga,
    };
    if let Err(errors) = data.validasi() {
        return Err(error_response(StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let gudang_id = match akun.role.as_deref() {
        Some("admin") => {
            let Some(gudang_id) = permintaan.gudang_id else {
                return Err(error_response(
                    StatusCode::BAD_REQUEST,
                    "gudang_id wajib diisi oleh admin",
                ));
            };

            match Barang::gudang_ada(&pool, gudang_id).await {
                Ok(true) => gudang_id,
                Ok(false) => {
                    return Err(error_response(
                        StatusCode::BAD_REQUEST,
                        format!("Gudang dengan ID {gudang_id} tidak ditemukan"),
                    ));
                }
                Err(error) => {
                    eprintln!("Gagal memeriksa gudang: {error}");
                    return Err(error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Gagal memeriksa gudang",
                    ));
                }
            }
        }
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => gudang_id,
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
                "Akun tidak memiliki hak untuk menambahkan barang",
            ));
        }
    };

    let barang = Barang::buat(&pool, data, gudang_id)
        .await
        .map_err(|error| {
            eprintln!("Gagal menambahkan barang: {error}");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal menambahkan barang",
            )
        })?;

    Ok((
        StatusCode::CREATED,
        barang_view::detail_barang_response(barang),
    ))
}

pub async fn ubah_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    path: Result<Path<i32>, PathRejection>,
    body: Result<Json<BarangBaru>, JsonRejection>,
) -> Result<Json<barang_view::BarangResponse>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let Json(data) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    if let Err(errors) = data.validasi() {
        return Err(error_response(StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let hasil = match akun.role.as_deref() {
        Some("admin") => Barang::ubah(&pool, id, data).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => Barang::ubah_di_gudang(&pool, id, gudang_id, data).await,
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
                "Akun tidak memiliki hak untuk mengubah barang",
            ));
        }
    };

    let barang = hasil.map_err(|error| {
        eprintln!("Gagal mengubah barang: {error}");
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Gagal mengubah barang")
    })?;

    match barang {
        Some(barang) => Ok(barang_view::detail_barang_response(barang)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn patch_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    path: Result<Path<i32>, PathRejection>,
    body: Result<Json<BarangUpdate>, JsonRejection>,
) -> Result<Json<barang_view::BarangResponse>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let Json(data) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    if data.nama.is_none() && data.stok.is_none() && data.harga.is_none() {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "PATCH harus menyertakan minimal satu field untuk diperbarui",
        ));
    }

    // Validasi hanya kolom yang dikirim pengguna.
    if data
        .nama
        .as_ref()
        .is_some_and(|nama| nama.trim().is_empty())
    {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "Nama barang tidak boleh kosong",
        ));
    }

    if data.stok.is_some_and(|stok| stok < 0) {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "Stok barang tidak boleh negatif",
        ));
    }

    if data
        .harga
        .is_some_and(|harga| harga < rust_decimal::Decimal::ZERO)
    {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "Harga barang tidak boleh negatif",
        ));
    }

    let hasil = match akun.role.as_deref() {
        Some("admin") => Barang::patch(&pool, id, data).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => Barang::patch_di_gudang(&pool, id, gudang_id, data).await,
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
                "Akun tidak memiliki hak untuk memperbarui barang",
            ));
        }
    };

    let barang = hasil.map_err(|error| {
        eprintln!("Gagal memperbarui sebagian data barang: {error}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal memperbarui data barang",
        )
    })?;

    match barang {
        Some(barang) => Ok(barang_view::detail_barang_response(barang)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn hapus_barang(
    State(pool): State<PgPool>,
    Extension(akun): Extension<AkunTerverifikasi>,
    path: Result<Path<i32>, PathRejection>,
) -> Result<(StatusCode, Json<PesanResponse>), Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let hasil = match akun.role.as_deref() {
        Some("admin") => Barang::hapus(&pool, id).await,
        Some("petugas_gudang") => match akun.gudang_id {
            Some(gudang_id) => Barang::hapus_di_gudang(&pool, id, gudang_id).await,
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
                "Akun tidak memiliki hak untuk menghapus barang",
            ));
        }
    };

    let terhapus = hasil.map_err(|error| {
        eprintln!("Gagal menghapus barang: {error}");
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Gagal menghapus barang")
    })?;

    if terhapus {
        Ok((
            StatusCode::OK,
            Json(PesanResponse {
                message: "Berhasil menghapus barang".to_string(),
            }),
        ))
    } else {
        Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Barang dengan ID {id} tidak ditemukan"),
        ))
    }
}
