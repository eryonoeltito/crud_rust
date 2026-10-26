use crate::models::barang::Barang;
use crate::views::format::format_nominal;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BarangResponse {
    pub id: i32,
    pub nama: String,
    pub stok: i32,
    pub harga: String,
}

impl From<Barang> for BarangResponse {
    fn from(item: Barang) -> Self {
        Self {
            id: item.id,
            nama: item.nama,
            stok: item.stok,
            harga: format_nominal(item.harga),
        }
    }
}

pub fn list_barang_response(barang: Vec<Barang>) -> Json<Vec<BarangResponse>> {
    let data = barang.into_iter().map(BarangResponse::from).collect();

    Json(data)
}

pub fn detail_barang_response(item: Barang) -> Json<BarangResponse> {
    Json(BarangResponse::from(item))
}
