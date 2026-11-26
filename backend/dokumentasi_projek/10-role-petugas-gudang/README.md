# 10 — Role Petugas Gudang

Petugas gudang adalah pengguna aktif dengan role `petugas_gudang` dan satu `gudang_id` yang ditetapkan pada akunnya. Petugas mengelola barang dan memantau perubahan stok untuk gudang tugasnya.

## Tujuan Pembelajaran

- Menjelaskan batas akses petugas berdasarkan gudang yang ditugaskan.
- Mengidentifikasi operasi barang dan riwayat stok yang tersedia bagi petugas.
- Menjelaskan mengapa `gudang_id` harus berasal dari akun terverifikasi di database.
- Menelusuri alur petugas saat melihat barang dan riwayat gudangnya.

## Hak akses berdasarkan gudang

Semua route barang dilindungi middleware JWT. Setelah token valid, middleware mengambil status aktif, role, dan `gudang_id` terbaru dari database, lalu menyimpannya sebagai `AkunTerverifikasi`. Handler menggunakan data akun tersebut untuk membatasi query ke gudang petugas.

| Fitur | Akses petugas gudang |
|---|---|
| Daftar barang | Hanya barang dari gudang tugasnya |
| Detail barang | Hanya barang dari gudang tugasnya |
| Membuat barang | Barang baru ditempatkan di gudang tugasnya |
| Mengubah, memperbarui sebagian, atau menghapus barang | Hanya barang dari gudang tugasnya |
| `GET /api/riwayat-stok` | Riwayat dengan snapshot `gudang_id` yang sama dengan gudang tugasnya; catatan barang terhapus tetap disertakan |
| `GET /api/barang/{id}/riwayat-stok` | Riwayat hanya jika barang berada di gudang tugasnya |

Untuk barang, API menyediakan `GET /api/barang`, `GET /api/barang/{id}`, `POST /api/barang`, `PUT /api/barang/{id}`, `PATCH /api/barang/{id}`, dan `DELETE /api/barang/{id}`. Untuk riwayat, gunakan `GET /api/riwayat-stok` atau `GET /api/barang/{id}/riwayat-stok`. Semua request tersebut memerlukan `Authorization: Bearer <token-petugas>`.

`gudang_id` yang dipakai untuk otorisasi berasal dari `AkunTerverifikasi` hasil pemeriksaan database, bukan dari parameter request, body JSON, atau claim JWT. Saat petugas membuat barang, handler menggunakan gudang pada akun terverifikasi; petugas tidak dapat memilih gudang lain dengan mengirim `gudang_id` sendiri.

JWT dapat memuat claim `role: "petugas_gudang"` sebagai informasi role saat token diterbitkan. Middleware tidak memakai claim itu untuk memberi akses; role dan `gudang_id` terbaru tetap diambil dari database pada setiap request. Perubahan penugasan atau role berlaku pada request berikutnya walaupun token masih memuat informasi lama.

Jika petugas meminta detail, mengubah, atau menghapus barang di gudang lain, query hanya mencari barang di gudang tugasnya. Hasilnya `404 Not Found`, sama seperti saat ID barang memang tidak ditemukan. Pemeriksaan barang untuk riwayat satu barang juga membatasi gudang dan memberikan 404 yang sama, sehingga petugas tidak mendapat petunjuk apakah barang itu ada di gudang lain.

Trigger membuat catatan ketika stok berubah melalui UPDATE, dengan menyimpan stok lama dan baru serta snapshot nama barang dan gudang. Daftar riwayat petugas difilter menggunakan `riwayat_stok.gudang_id` yang dicocokkan dengan gudang dari akun terverifikasi; nilai yang tidak cocok tidak masuk daftar. Saat barang dihapus, foreign key mengubah `barang_id` riwayat menjadi `NULL`, sedangkan snapshot nama barang dan gudang tetap tersedia, sehingga catatan itu masih dapat dibaca petugas gudang terkait.

## Contoh alur penggunaan

1. Petugas login melalui `POST /api/auth/login`. Akun harus aktif agar login dapat menerbitkan JWT.
2. Petugas meminta `GET /api/barang` dengan Bearer token dan hanya menerima daftar barang gudangnya.
3. Petugas menambahkan barang melalui `POST /api/barang`; gudang tujuan diambil dari akun terverifikasi.
4. Petugas membuka `GET /api/barang/15/riwayat-stok` untuk melihat perubahan stok jika barang 15 berada di gudangnya.

Contoh body untuk menambahkan barang tidak memuat `gudang_id`:

```json
{
  "nama": "Kabel HDMI",
  "stok": 12,
  "harga": 75000
}
```
