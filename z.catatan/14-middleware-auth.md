# 14 — Penjelasan kode `src/middleware/auth_middleware.rs`

Middleware adalah fungsi yang memeriksa request sebelum request diteruskan ke handler endpoint. Dalam proyek ini, `wajib_login` memeriksa apakah session menyimpan username.

## Import

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tower_sessions::Session;
```

- `Request`: tipe request HTTP yang sedang diproses.
- `StatusCode`: untuk menetapkan status HTTP.
- `Next`: mewakili langkah berikutnya dalam rantai middleware menuju handler.
- `IntoResponse`: mengubah tuple status dan body JSON menjadi respons HTTP.
- `Response`: tipe respons HTTP yang dikembalikan middleware.
- `Json` dan `json!`: menyusun body respons dalam format JSON.
- `Session`: membaca session pengguna.

## Fungsi `wajib_login`

```rust
pub async fn wajib_login(
    session: Session,
    request: Request,
    next: Next,
) -> Response {
```

- `pub`: fungsi dapat dipakai dari router.
- `async`: fungsi dapat menunggu operasi session.
- `session`: session dari request.
- `request`: request yang akan diperiksa dan mungkin diteruskan.
- `next`: mekanisme untuk melanjutkan ke handler berikutnya.
- `-> Response`: middleware menghasilkan respons HTTP.

```rust
    let username = match session.get::<String>("username").await {
```
- `session.get::<String>("username")` mencoba membaca item bernama `username` dari session sebagai `String`.
- `.await` menunggu operasi baca.
- `match` membedakan nilai yang berhasil dibaca dari kegagalan penyimpanan.

```rust
        Ok(username) => username,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Gagal memeriksa session"
                })),
            ).into_response();
        }
    };
```
- `Ok(username)` berarti pembacaan berhasil; nilai bisa berupa `Some(...)` atau `None`.
- `Err(_)` berarti ada kegagalan saat membaca session.
- Status `500` menandakan kegagalan internal.
- `.into_response()` mengubah status dan JSON menjadi satu respons.

```rust
    match username {
        Some(_) => next.run(request).await,
        None => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Silakan login terlebih dahulu"
            })),
        ).into_response(),
    }
}
```
- `Some(_)`: item username ditemukan, jadi request dilanjutkan ke handler melalui `next.run(request).await`.
- `None`: username tidak ada dalam session, jadi middleware menghentikan request dan mengembalikan `401 Unauthorized`.
- Pesan JSON memberi tahu client bahwa login diperlukan.

## Mengapa `wajib_login` dipasang pada router barang?

Router barang memiliki endpoint daftar, detail, tambah, ubah, dan hapus. Dengan `route_layer(middleware::from_fn(wajib_login))`, semua route dalam router tersebut diperiksa oleh middleware. Route `/login` tidak dilindungi middleware ini agar pengguna bisa masuk.

## Contoh respons tanpa session

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json
```

```json
{
  "status": "error",
  "message": "Silakan login terlebih dahulu"
}
```
