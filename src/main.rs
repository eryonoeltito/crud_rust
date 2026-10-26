mod controllers;
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

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Gagal membuka port");

    println!("Server berjalan di http://127.0.0.1:3000");

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
