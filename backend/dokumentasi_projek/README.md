# Dokumentasi API Inventaris

Dokumentasi ini mengikuti implementasi yang berada di `src/`. Aplikasi memakai Axum, SQLx PostgreSQL, Argon2, JWT HS256, dan pembagian MVC sederhana.

## Peta materi

| Bagian | Topik |
|---|---|
| 00 | [Koneksi Database](00-koneksi-database/README.md) |
| 01 | [Routes dan Handler](01-routes-handler/README.md) |
| 02 | [Listener](02-listener/README.md) |
| 03 | [CRUD Barang](03-crud-barang/README.md) |
| 04 | [Format Harga](04-format-harga/README.md) |
| 05 | [JSON Array](05-json-array/README.md) |
| 06 | [Login dan Logout](06-login-logout/README.md) |
| 07 | [JWT dan Middleware](07-session-cookie-middleware/README.md) |
| 08 | [JWT HMAC](08-jwt-hmac/README.md) |
| 09 | [Role Admin](09-role-admin/README.md) |
| 10 | [Role Petugas Gudang](10-role-petugas-gudang/README.md) |

## Arsitektur dan alur

```text
Client → TcpListener → Router → wajib_login (route terlindungi)
       → Controller → Model/SQLx → PostgreSQL
       ← JSON response ← View/Controller
```

- `routes/` memetakan method dan path.
- `controllers/` menangani input, validasi, dan status response.
- `models/` menjalankan query PostgreSQL dengan SQLx.
- `middleware/` memvalidasi JWT dan mengambil akun terbaru.
- `views/` memformat response barang dan error.
- `auth.rs` membuat serta memvalidasi JWT.
- `main.rs` membuat pool, memasang Axum `State`/`Extension`, dan menjalankan listener.

## Daftar endpoint

| Method | Path | Perlindungan |
|---|---|---|
| GET | `/health` | Tidak perlu token |
| POST | `/register` | Tidak perlu token |
| POST | `/login` | Tidak perlu token |
| POST | `/logout` | Bearer JWT divalidasi handler |
| GET, POST | `/barang` | JWT + akun valid |
| GET, PUT, PATCH, DELETE | `/barang/{id}` | JWT + akun valid |
| GET | `/riwayat-stok` | JWT + akun valid |
| GET | `/barang/{id}/riwayat-stok` | JWT + akun valid |

## Format error

Error API berbentuk objek berikut:

```json
{"status":"error","message":"pesan kesalahan"}
```

Status dan pesan bergantung pada validasi, autentikasi, otorisasi, data yang tidak ditemukan, atau kegagalan database. Tidak ada klaim bahwa endpoint atau pengujian yang tidak terlihat di source code sudah tersedia atau berhasil.

Untuk contoh request, gunakan endpoint pada bagian 03, 06, 09, dan 10. Pengujian runtime harus dilakukan pada database uji yang terpisah dan tidak didokumentasikan sebagai berhasil tanpa bukti hasilnya.
