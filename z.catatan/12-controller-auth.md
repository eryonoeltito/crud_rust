# 12 — Penjelasan kode `src/controllers/auth_controller.rs`

Controller ini menangani request login dan logout. Penjelasan mengikuti struktur implementasi session yang digunakan dalam proyek.

## Import dan tipe request

```rust
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;
use crate::models::user::User;
```

- `State`: mengambil state aplikasi, di sini pool database.
- `StatusCode`: nama status HTTP seperti `200`, `401`, dan `500`.
- `Json`: mengekstrak body JSON dari request dan membungkus JSON untuk response.
- `Deserialize`: memungkinkan JSON diubah menjadi struct Rust.
- `json!`: makro untuk membuat nilai JSON.
- `PgPool`: tipe pool koneksi PostgreSQL.
- `Session`: tipe session dari `tower-sessions`.
- `User`: model yang menyediakan fungsi pemeriksaan kredensial.

## Bentuk body login

```rust
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
```

- `#[derive(Deserialize)]`: membuat implementasi deserialisasi otomatis.
- `pub struct LoginRequest`: tipe data request login yang dapat dipakai dari modul lain.
- `username: String`: teks username yang dikirim client.
- `password: String`: teks password yang dikirim client.
- Kedua field `pub` agar dapat diakses sesuai kebutuhan modul.

Contoh body:
```json
{"username":"admin","password":"admin123"}
```

## Handler `login`

```rust
pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(input): Json<LoginRequest>,
) -> axum::response::Response {
```

- `pub`: route dapat merujuk fungsi ini dari modul routes.
- `async`: handler dapat menunggu operasi database dan session.
- `State(pool)`: mengambil pool database dari state router.
- `session: Session`: menerima session yang disediakan session middleware.
- `Json(input)`: membaca JSON dan mengubahnya menjadi `LoginRequest`.
- `-> Response`: handler mengembalikan respons HTTP yang bisa berisi status dan JSON.

```rust
    let valid = match User::cek_login(
        &pool,
        &input.username,
        &input.password,
    ).await {
```
- `let valid` menyiapkan hasil pemeriksaan.
- `User::cek_login(...)` memanggil model untuk memeriksa kredensial.
- `&pool`, `&input.username`, `&input.password` mengirim referensi ke data yang dibutuhkan.
- `.await` menunggu hasil query.
- `match` memisahkan hasil sukses dan error.

```rust
        Ok(valid) => valid,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Terjadi kesalahan pada server"
                })),
            ).into_response();
        }
    };
```
- `Ok(valid) => valid`: memakai nilai boolean dari query.
- `Err(_)`: menangani error database tanpa menampilkan detail internal kepada client.
- `return`: langsung mengakhiri handler dengan respons error.
- `500 Internal Server Error`: menandakan kegagalan internal, bukan sekadar password salah.
- `Json(json!(...))`: membentuk respons JSON.
- `.into_response()`: mengubah tuple status dan JSON menjadi tipe `Response`.

```rust
    if !valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Username atau password salah"
            })),
        ).into_response();
    }
```
- `!valid` berarti hasil pemeriksaan `false`.
- Status `401 Unauthorized` dipakai saat kredensial tidak cocok.
- Satu pesan untuk kedua kemungkinan menghindari memberi tahu apakah username tertentu terdaftar.

```rust
    if session.insert("username", input.username).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Gagal membuat session"
            })),
        ).into_response();
    }
```
- `session.insert("username", input.username)` menyimpan username dalam data session.
- `.await` menunggu operasi session.
- `.is_err()` memeriksa apakah operasi gagal.
- Jika gagal, handler mengembalikan `500` dan pesan yang sesuai.

```rust
    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Login berhasil",
            "session": "aktif"
        })),
    ).into_response()
}
```
- Jika semua tahap berhasil, responsnya `200 OK`.
- JSON memberi pesan bahwa login berhasil.
- `session: "aktif"` hanya keterangan dalam respons; validitas sebenarnya dibuktikan dengan session pada request berikutnya.

## Handler `logout`

Bentuk fungsi logout yang digunakan pada proyek:

```rust
pub async fn logout(
    session: tower_sessions::Session,
) -> Result<Json<serde_json::Value>, StatusCode> {
    session
        .flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": "Logout berhasil"
    })))
}
```

- `session: tower_sessions::Session`: menerima session yang sedang digunakan.
- `session.flush().await`: menghapus data session dan mengakhiri session aktif.
- `map_err(...)`: mengubah kegagalan operasi menjadi status `500`.
- `?`: mengembalikan error lebih awal jika `flush` gagal.
- `Ok(Json(...))`: mengembalikan JSON sukses bila logout berhasil.

## Status respons utama

| Kondisi | Status |
|---|---:|
| Login berhasil | `200 OK` |
| Username/password tidak cocok | `401 Unauthorized` |
| Query database atau operasi session gagal | `500 Internal Server Error` |
| Logout berhasil | `200 OK` |
