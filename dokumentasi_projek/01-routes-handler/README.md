# 01 — Routes dan Handler

Route menghubungkan method dan path HTTP dengan fungsi controller. Controller mengambil input, melakukan validasi, memanggil model, lalu membentuk response.

## Pendaftaran route

Router dari `src/routes/` digabungkan di `src/main.rs`:

| Method | Path | Fungsi |
|---|---|---|
| GET | `/health` | pemeriksaan server |
| POST | `/register` | registrasi |
| POST | `/login` | login dan penerbitan JWT |
| POST | `/logout` | validasi JWT untuk logout |
| GET, POST | `/barang` | daftar dan membuat barang |
| GET, PUT, PATCH, DELETE | `/barang/{id}` | detail dan perubahan barang |
| GET | `/riwayat-stok` | daftar riwayat |
| GET | `/barang/{id}/riwayat-stok` | riwayat satu barang |

Route barang dan riwayat diberi `route_layer(middleware::from_fn(wajib_login))`. Register, login, dan logout tidak berada pada lapisan itu. Logout tetap memeriksa Bearer token di handler.

## State dan Extension

`.with_state(pool)` memasang `PgPool` sebagai state Axum. Controller mengambilnya dengan `State(pool)`. `Extension(jwt_secret)` mengambil secret JWT yang dipasang di router. Middleware menambahkan `AkunTerverifikasi` ke request extensions setelah token dan akun database lolos pemeriksaan; controller mengambilnya dengan `Extension(akun)`.

Extractor lain yang dipakai adalah `Path(id)` untuk parameter URL dan `Json(data)` untuk body JSON. Body atau path yang tidak dapat diparsing menghasilkan response error sesuai status rejection Axum.

## Alur request

```text
Client → listener → router → middleware (jika terlindungi)
       → controller → model/SQLx → view atau JSON response → Client
```

Response error memiliki bentuk yang sama:

```json
{"status":"error","message":"pesan kesalahan"}
```

Status yang digunakan antara lain `200 OK`, `201 Created`, `400 Bad Request`, `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `409 Conflict`, dan `500 Internal Server Error`. Detail error database hanya ditulis ke log server; client menerima pesan umum.
