mod controllers;
mod database;
mod middleware;
mod models;
mod routes;
mod views;

use axum::{Json, Router, routing::get};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::routes::{auth_routes, barang_routes};

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

    database::initialize(&pool)
        .await
        .unwrap_or_else(|error| panic!("Inisialisasi database gagal: {error}"));
    println!("Tabel `barang` dan `users` siap digunakan.");

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(barang_routes::routes())
        .merge(auth_routes::routes())
        .with_state(pool)
        .layer(session_layer);

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
