mod auth;
mod controllers;
mod middleware;
mod models;
mod routes;
mod views;

use std::sync::Arc;

use axum::{Json, Router, extract::Extension, http::StatusCode, routing::get};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::routes::{auth_routes, barang_routes, riwayat_stok_routes};
use crate::views::error_view::error_response;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL tidak ditemukan di file .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Gagal terhubung ke PostgreSQL");

    println!("Berhasil terhubung ke PostgreSQL!");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET tidak ditemukan di file .env");
    if jwt_secret.as_bytes().len() < 32 {
        panic!("JWT_SECRET harus memiliki minimal 32 byte untuk HS256");
    }
    let jwt_secret: Arc<str> = Arc::from(jwt_secret);

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(barang_routes::routes())
        .merge(riwayat_stok_routes::routes())
        .merge(auth_routes::routes())
        .fallback(|| async { error_response(StatusCode::NOT_FOUND, "Route tidak ditemukan") })
        .method_not_allowed_fallback(|| async {
            error_response(StatusCode::METHOD_NOT_ALLOWED, "Method tidak diizinkan")
        })
        .layer(Extension(pool.clone()))
        .with_state(pool)
        .layer(Extension(jwt_secret));

    let app_host = std::env::var("APP_HOST").expect("APP_HOST tidak ditemukan di file .env");

    let app_port = std::env::var("APP_PORT")
        .expect("APP_PORT tidak ditemukan di file .env")
        .parse::<u16>()
        .expect("APP_PORT harus berupa angka antara 0 hingga 65535");

    let listener = TcpListener::bind((app_host.as_str(), app_port))
        .await
        .unwrap_or_else(|error| {
            panic!("Gagal membuka listener pada {app_host}:{app_port}: {error}")
        });

    let address = listener
        .local_addr()
        .expect("Gagal membaca alamat listener");
    println!("Server berjalan di http://{address}");

    axum::serve(listener, app)
        .await
        .expect("Server gagal berjalan");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message": "API inventaris berhasil berjalan"
    }))
}