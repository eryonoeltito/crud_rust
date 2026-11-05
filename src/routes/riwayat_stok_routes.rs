use axum::{Router, middleware, routing::get};
use sqlx::PgPool;

use crate::{controllers::riwayat_stok_controller, middleware::auth_middleware::wajib_login};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/riwayat-stok",
            get(riwayat_stok_controller::list_riwayat_stok),
        )
        .route(
            "/barang/{id}/riwayat-stok",
            get(riwayat_stok_controller::list_riwayat_barang),
        )
        .route_layer(middleware::from_fn(wajib_login))
}
