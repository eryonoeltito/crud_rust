# 13 — Penjelasan kode `src/routes/auth_routes.rs`

File route menghubungkan alamat URL dan metode HTTP dengan fungsi controller.

## Kode sumber

```rust
use axum::{routing::post, Router};
use sqlx::PgPool;
use crate::controllers::auth_controller;
```

- `routing::post`: fungsi untuk mendaftarkan route yang menerima metode HTTP POST.
- `Router`: tipe router Axum untuk menyusun daftar route.
- `PgPool`: tipe state aplikasi berupa pool database PostgreSQL.
- `auth_controller`: modul yang berisi handler register, login, dan logout.

```rust
pub fn routes() -> Router<PgPool> {
```
- `pub`: fungsi bisa dipanggil dari modul lain, seperti `main.rs`.
- `fn routes()`: membuat dan mengembalikan kumpulan route autentikasi.
- `Router<PgPool>`: router ini memakai `PgPool` sebagai tipe state yang dibutuhkan handler.

```rust
    Router::new()
        .route("/register", post(auth_controller::register))
        .route("/login", post(auth_controller::login))
        .route("/logout", post(auth_controller::logout))
}
```
- `Router::new()` membuat router kosong.
- `.route("/register", post(auth_controller::register))` menghubungkan `POST /register` ke handler pendaftaran akun.
- `.route("/login", post(auth_controller::login))` menghubungkan `POST /login` ke fungsi `login`.
- `.route("/logout", post(auth_controller::logout))` menghubungkan `POST /logout` ke fungsi `logout`.
- `}` menutup fungsi.

## Mengapa route login tidak memakai `wajib_login`?

Pengguna belum memiliki session saat hendak login. Karena itu route autentikasi dipisahkan dari router barang yang dipasangi middleware `wajib_login`. Jika login ikut dilindungi middleware yang sama, pengguna baru akan terhalang sebelum bisa login.

## Ringkasan

| Metode | Path | Fungsi |
|---|---|---|
| `POST` | `/register` | Mendaftarkan akun baru |
| `POST` | `/login` | Memeriksa kredensial dan membuat session |
| `POST` | `/logout` | Mengakhiri session aktif |
