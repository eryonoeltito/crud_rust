# 03 — CRUD Barang

CRUD adalah operasi **Create, Read, Update, Delete** (membuat, membaca, mengubah, menghapus). Fitur ini memungkinkan client mengelola data inventaris tanpa mengakses database secara langsung. Model data dan query berada di `src/models/barang.rs`, handler di `src/controllers/barang_controller.rs`, pemetaan URL di `src/routes/barang_routes.rs`, dan bentuk response disiapkan oleh `src/views/barang_view.rs`.

## Endpoint

| Method | Path | Operasi |
|---|---|---|
| GET | `/barang` | Membaca daftar barang |
| GET | `/barang/{id}` | Membaca satu barang |
| POST | `/barang` | Membuat barang |
| PUT | `/barang/{id}` | Mengganti nama, stok, dan harga |
| PATCH | `/barang/{id}` | Mengubah sebagian field |
| DELETE | `/barang/{id}` | Menghapus barang |

Semua route barang memerlukan session login. Rinciannya dijelaskan di [07 — Session, Cookie, dan Middleware](../07-session-cookie-middleware/README.md).

Contoh request untuk membuat barang:

```http
POST /barang
Content-Type: application/json

{"nama":"Keyboard","stok":8,"harga":250000}
```

Request ini harus disertai cookie session login. Jika valid, server mengembalikan `201 Created` beserta data barang yang baru disimpan.

## Alur operasi

Alur setiap operasi dimulai saat handler mengambil data dari path atau JSON. Handler memvalidasi input sebelum memanggil fungsi model yang menjalankan query SQLx. Query memakai placeholder seperti `$1` dan `.bind(...)` untuk mengirim nilai sebagai parameter, sehingga nilai input tidak digabungkan langsung ke teks SQL. Hasil model dikonversi menjadi respons; ID yang tidak ditemukan menghasilkan `404`.

Validasi menolak nama kosong (termasuk nama yang hanya berisi spasi), stok negatif, dan harga negatif. PUT meminta data lengkap karena maknanya mengganti seluruh field yang dikelola. PATCH menerima field opsional untuk perubahan sebagian; field yang tidak dikirim dipertahankan melalui `COALESCE` di query, dan body kosong ditolak karena tidak ada perubahan yang diminta. Harga disimpan sebagai `NUMERIC` PostgreSQL dan dipetakan ke `Decimal` agar tidak melewati floating point.

Operasi baca daftar menghasilkan koleksi `Vec<Barang>`, sementara pencarian ID menghasilkan `Option<Barang>`: `Some` berarti ditemukan dan `None` berarti tidak ada. DELETE menghasilkan nilai boolean yang menunjukkan apakah baris terhapus. Respons daftar berbentuk JSON array berisi objek barang. Lihat [05 — Objek JSON berupa JSON Array](../05-json-array/README.md) dan [04 — Format Harga](../04-format-harga/README.md).
