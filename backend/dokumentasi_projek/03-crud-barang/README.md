# 03 — CRUD Barang

CRUD berarti membuat, membaca, mengubah, dan menghapus data barang. Query berada di `src/models/barang.rs`, controller di `src/controllers/barang_controller.rs`, route di `src/routes/barang_routes.rs`, dan bentuk response di `src/views/barang_view.rs`.

## Endpoint

| Method | Path | Berhasil | Keterangan |
|---|---|---|---|
| GET | `/api/barang` | 200 | Admin melihat semua; petugas hanya gudangnya |
| GET | `/api/barang/{id}` | 200 | Detail barang sesuai hak akses |
| POST | `/api/barang` | 201 | Membuat barang |
| PUT | `/api/barang/{id}` | 200 | Mengganti nama, stok, dan harga |
| PATCH | `/api/barang/{id}` | 200 | Mengubah field yang dikirim |
| DELETE | `/api/barang/{id}` | 200 | Menghapus barang |

Semua endpoint di atas memerlukan `Authorization: Bearer <token>`.

## Request dan response

Body POST:

```json
{"nama":"Keyboard","stok":8,"harga":250000,"gudang_id":2}
```

`gudang_id` pada POST hanya dipakai admin untuk memilih gudang dan wajib menunjuk gudang yang ada. Petugas tidak diberi gudang dari body; handler memakai `gudang_id` milik akun terverifikasi.

Body PUT:

```json
{"nama":"Keyboard","stok":8,"harga":250000}
```

Body PUT dipetakan ke `BarangBaru`, sehingga `nama`, `stok`, dan `harga` wajib ada. Body PATCH memakai field opsional berikut:

```json
{"stok":12}
```

PATCH menolak body tanpa field, nama kosong, stok negatif, atau harga negatif. POST/PUT menolak nama kosong, stok negatif, dan harga negatif. POST admin juga menolak `gudang_id` yang tidak diisi atau tidak ditemukan. Response barang memakai harga string:

```json
{"id":1,"nama":"Keyboard","stok":8,"harga":"Rp250.000,00"}
```

DELETE yang berhasil mengembalikan `200 OK` dengan body `{"message":"Berhasil menghapus barang"}`. Token tidak ada atau tidak valid menghasilkan `401`; role tidak berhak atau akun petugas tanpa gudang menghasilkan `403`. Jika ID tidak ditemukan atau berada di gudang lain bagi petugas, response adalah `404 Not Found`. Body/path yang tidak valid memakai status rejection Axum, input bisnis tidak valid memakai `400`, dan kegagalan query memakai `500`.

## Alur dan keamanan query

Controller mengambil `State<PgPool>` dan `Extension<AkunTerverifikasi>`, memvalidasi input, lalu memanggil model. Model memakai placeholder SQL seperti `$1` dan `.bind(...)`; nilai input tidak digabungkan ke teks SQL. Query petugas menambahkan syarat `gudang_id`, sedangkan query admin tidak membatasi gudang.

Operasi baca daftar memakai `fetch_all` dan menghasilkan array JSON. Operasi detail memakai `fetch_optional`, sehingga `None` diterjemahkan menjadi 404. Harga disimpan sebagai `NUMERIC` dan diformat oleh view; penjelasan format ada di [04 — Format Harga](../04-format-harga/README.md).
