# 01 — Routes dan Handler

Route menghubungkan method dan path HTTP dengan fungsi controller. Controller mengambil input, melakukan validasi, memanggil model, lalu membentuk response.

## Pendaftaran route

Router dari `src/routes/` digabungkan di `src/main.rs`:

| Method | Path | Fungsi |
|---|---|---|
| GET | `/health` | pemeriksaan server |
| POST | `/api/auth/register` | registrasi |
| POST | `/api/auth/login` | login dan penerbitan JWT |
| POST | `/api/auth/logout` | validasi JWT untuk logout |
| GET, POST | `/api/barang` | daftar dan membuat barang |
| GET, PUT, PATCH, DELETE | `/api/barang/{id}` | detail dan perubahan barang |
| GET | `/api/riwayat-stok` | daftar riwayat |
| GET | `/api/barang/{id}/riwayat-stok` | riwayat satu barang |

Route barang dan riwayat diberi `route_layer(middleware::from_fn(wajib_login))`. Register, login, dan logout tidak berada pada lapisan itu. Logout tetap memeriksa Bearer token di handler.

## State dan Extension

`.with_state(pool)` memasang `PgPool` sebagai state Axum. Controller mengambilnya dengan `State(pool)`. `Extension(jwt_secret)` mengambil secret JWT yang dipasang di router. Middleware menambahkan `AkunTerverifikasi` ke request extensions setelah token dan akun database lolos pemeriksaan; controller mengambilnya dengan `Extension(akun)`.

Extractor lain yang dipakai adalah `Path(id)` untuk parameter URL dan `Json(data)` untuk body JSON. Handler logout memakai `HeaderMap` untuk membaca header `Authorization`. Body atau path yang tidak dapat diparsing menghasilkan response error dengan status rejection Axum.

## Alur request

```text
Client → listener → router → middleware (jika terlindungi)
       → controller → model/SQLx → view atau JSON response → Client
```

Alur tiap route dapat berbeda: `/health` langsung mengembalikan JSON dari handler; register dan login memakai model untuk mengakses database; logout memvalidasi JWT di handler tanpa query database. Route barang dan riwayat melewati middleware sebelum controller dan model.

Response error memiliki bentuk yang sama:

```json
{"status":"error","message":"pesan kesalahan"}
```

Status `200 OK` digunakan untuk operasi sukses seperti membaca, mengubah, menghapus, login, dan logout; `201 Created` untuk register dan membuat barang. `400 Bad Request` menandai input bisnis yang tidak valid; `401 Unauthorized` mencakup kredensial login atau token yang tidak valid; `403 Forbidden` mencakup akun tidak aktif atau akses yang tidak diizinkan; `404 Not Found` untuk data atau route yang tidak ditemukan; `405 Method Not Allowed` untuk method yang tidak didukung; dan `409 Conflict` untuk username yang sudah digunakan. Kegagalan database atau internal menghasilkan `500 Internal Server Error`. Body error memakai bentuk JSON yang ditampilkan di atas. Detail error database hanya ditulis ke log server; client menerima pesan umum.
