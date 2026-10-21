
use axum::Json;
use crate::models::barang::Barang;
use crate::views::format::format_nominal;

    pub fn list_barang_response(
        barang: Vec<Barang>,
    ) -> Json<Vec<Vec<serde_json::Value>>> {
        let data = barang
            .into_iter()
            .map(|item| {
                vec![
                    serde_json::json!(item.id),
                    serde_json::json!(item.nama),
                    serde_json::json!(item.stok),
                    serde_json::json!(format_nominal(item.harga)),
                ]
            })
            .collect();

        Json(data)
    }

    pub fn detail_barang_response(
        item: Barang,
    ) -> Json<Vec<serde_json::Value>> {
        Json(vec![
            serde_json::json!(item.id),
            serde_json::json!(item.nama),
            serde_json::json!(item.stok),
            serde_json::json!(format_nominal(item.harga)),
        ])
    }

