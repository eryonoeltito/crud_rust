
// mod models;
// mod controllers;
// mod views;
// mod routes;

// use axum::{
//     extract::State,
//     http::StatusCode,
//     routing::get,
//     Json,
//     Router,
// };
// use serde_json::json;
// use sqlx::postgres::PgPoolOptions;
// use tokio::net::TcpListener;

// use crate::routes::barang_routes;

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();

//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL tidak ditemukan di file .env");

//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .expect("Gagal terhubung ke PostgreSQL");

//     println!("Berhasil terhubung ke PostgreSQL!");

//     let app = Router::new()
//         .route("/health", get(health_check))
//         .route("/test-koneksi", get(test_koneksi))
//         .merge(barang_routes::routes())
//         .with_state(pool);

//     let listener = TcpListener::bind("127.0.0.1:3000")
//         .await
//         .expect("Gagal membuka port");

//     println!("Server berjalan di http://127.0.0.1:3000");

//     axum::serve(listener, app)
//         .await
//         .expect("Server gagal berjalan");
// }

// async fn health_check() -> Json<serde_json::Value> {
//     Json(json!({
//         "status": "success",
//         "message": "API inventaris berhasil berjalan"
//     }))
// }

// async fn test_koneksi(
//     State(pool): State<sqlx::PgPool>,
// ) -> Result<&'static str, (StatusCode, String)> {
//     sqlx::query("SELECT pg_sleep(5)")
//         .execute(&pool)
//         .await
//         .map_err(|error| {
//             eprintln!("Pengujian koneksi gagal: {error}");
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Pengujian koneksi gagal".to_string(),
//             )
//         })?;

//     Ok("Koneksi database selesai diuji")
// }
