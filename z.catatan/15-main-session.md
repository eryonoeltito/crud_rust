# 15 — Penjelasan konfigurasi session di `src/main.rs`

Dokumen ini berfokus pada bagian `main.rs` yang memasang session manager. Pertahankan bagian lain dari file utama, seperti konfigurasi database, alamat listener, dan pemanggilan `axum::serve`.

## Import terkait session

```rust
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
```

- `Expiry`: tipe untuk menentukan kapan cookie/session dianggap berakhir sesuai kebijakan expiry.
- `MemoryStore`: penyimpanan session di memori proses server.
- `SessionManagerLayer`: layer Tower yang mengelola session pada request dan response.

## Membuat penyimpanan dan layer session

```rust
let session_store = MemoryStore::default();
```
- `let` membuat variabel.
- `MemoryStore::default()` membuat penyimpanan session baru di memori.
- Data ini tidak persisten: session akan hilang saat server dimulai ulang.

```rust
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnSessionEnd);
```
- `SessionManagerLayer::new(session_store)` membuat layer session dengan store yang telah dibuat.
- `.with_secure(false)` membolehkan cookie dipakai pada HTTP lokal untuk pengembangan. Untuk aplikasi publik, gunakan HTTPS dan cookie Secure.
- `.with_expiry(Expiry::OnSessionEnd)` mengatur expiry cookie menjadi akhir sesi browser, bukan batas waktu tertentu yang dipilih aplikasi.
- Browser dapat memiliki fitur pemulihan sesi, jadi menutup jendela tidak selalu menjamin semua cookie sesi hilang selamanya.

```rust
let app = Router::new()
    .merge(auth_routes::routes())
    .merge(barang_routes::routes())
    .with_state(pool)
    .layer(session_layer);
```
- `Router::new()` membuat router utama.
- `.merge(auth_routes::routes())` menggabungkan route login/logout.
- `.merge(barang_routes::routes())` menggabungkan route CRUD barang.
- `.with_state(pool)` menyediakan pool PostgreSQL kepada handler.
- `.layer(session_layer)` memasang pengelola session pada aplikasi sehingga handler/middleware dapat menerima `Session`.

## Catatan konsep

Session dan cookie bukan hal yang sama:
- **Session**: data status login yang disimpan di sisi server.
- **Cookie**: cara client mengirim pengenal session ke server pada request berikutnya.
- **JWT**: token yang berisi klaim dan ditandatangani; proyek ini tidak menggunakan JWT.

Dengan `MemoryStore`, server dapat memeriksa session di sisi server. Cookie `id` hanya menjadi pengenal untuk mencari session yang tersimpan.

## Perhatian

Dokumentasi ini menjelaskan konfigurasi yang digunakan dalam proyek. Periksa lagi source aktif sebelum mengubah versi crate atau pengaturan expiry, karena API crate dapat berubah antarversi.
