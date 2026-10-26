# 02 — `Cargo.toml`: dependency dan alasannya

`Cargo.toml` adalah berkas konfigurasi project Rust. Bagian `[dependencies]` menyatakan crate yang dibutuhkan.

| Dependency | Untuk apa | Jika tidak dipakai |
|---|---|---|
| `axum` | Router, handler, ekstraksi request, respons HTTP | Perlu framework HTTP lain atau membangun banyak hal sendiri |
| `tokio` dengan `full` | Runtime async untuk server dan operasi async | Fungsi async tidak otomatis berjalan; perlu runtime yang sesuai |
| `sqlx` | Pool PostgreSQL dan query async | Harus memakai driver/library lain atau menulis akses DB sendiri |
| `dotenvy` | Membaca variabel dari `.env` saat development | `DATABASE_URL` harus disetel di environment secara manual |
| `serde` dengan `derive` | `Serialize`/`Deserialize` untuk mengubah data Rust ↔ format data | Perlu implementasi serialisasi/deserialisasi manual atau library lain |
| `serde_json` | Membuat nilai JSON lewat `json!` | Perlu membangun nilai JSON dengan cara lain |
| `rust_decimal` dengan fitur `serde` | Tipe `Decimal` untuk harga dan dukungan serialisasi/deserialisasi | Harga berisiko memakai floating point; tipe ini juga memungkinkan nilai harga berpindah melalui JSON |
| `rand` | Library bilangan acak | Di source saat ini belum terlihat dipakai; bisa jadi sisa dependency dari percobaan sebelumnya |

Fitur `rust_decimal` pada `sqlx` mengaktifkan pemetaan `Decimal` dengan tipe `NUMERIC` PostgreSQL.

Contoh `serde`:
```rust
#[derive(Deserialize)]
struct BarangBaru { nama: String, stok: i32, harga: Decimal }
```
Axum dapat membaca JSON request menjadi `BarangBaru` karena tipe tersebut mengimplementasikan `Deserialize`.

Contoh jika tanpa `Deserialize`: JSON tidak dapat otomatis diubah menjadi struct itu oleh extractor `Json<BarangBaru>`.

### `edition = "2024"`
Menentukan edisi bahasa Rust yang digunakan compiler. Ini bukan versi aplikasi. Mengubah edisi dapat memengaruhi aturan bahasa dan kompatibilitas, jadi jangan diganti tanpa alasan.

### Mengapa versi dependency ditulis?
Contoh `axum = "0.8.9"` memberi Cargo batas versi kompatibel menurut aturan versi Cargo; `Cargo.lock` merekam versi yang terpilih untuk build yang lebih konsisten. Jangan menghapus lockfile tanpa memahami dampaknya pada aplikasi binary.
