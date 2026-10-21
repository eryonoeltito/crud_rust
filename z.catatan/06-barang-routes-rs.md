# 06 — `src/routes/barang_routes.rs`

Route menghubungkan kombinasi HTTP method dan URL ke handler controller.

| Method + URL | Handler | Tujuan |
|---|---|---|
| `GET /barang` | `list_barang` | Daftar barang |
| `POST /barang` | `buat_barang` | Tambah barang |
| `GET /barang/{id}` | `detail_barang` | Lihat satu barang |
| `PUT /barang/{id}` | `ubah_barang` | Ganti data barang secara lengkap |
| `PATCH /barang/{id}` | `patch_barang` | Ubah sebagian field |
| `DELETE /barang/{id}` | `hapus_barang` | Hapus barang |

`get(...)` dan `put(...)` mendaftarkan handler untuk method tertentu. `.post()`, `.patch()`, `.delete()` menambahkan method pada route yang sama.

### Mengapa method berbeda?
Method menjelaskan maksud request. Jika semua operasi menggunakan GET, tindakan seperti menghapus atau mengubah data menjadi tidak jelas dan melanggar kebiasaan REST; GET seharusnya dipakai untuk membaca data.

### PUT vs PATCH
- PUT pada implementasi ini meminta `BarangBaru` berisi nama, stok, harga; artinya data yang dikelola diganti lengkap.
- PATCH menerima `BarangUpdate` dengan `Option`, jadi cukup kirim field yang ingin diubah.

Contoh PUT:
```json
{"nama":"Monitor","stok":10,"harga":1500000}
```
Contoh PATCH:
```json
{"stok":15}
```

Jika tidak membedakan PUT/PATCH, API bisa membingungkan client tentang apakah field yang tidak dikirim akan dihapus, ditolak, atau dipertahankan.

`/barang/{id}` menggunakan parameter dinamis. Angka ID tidak ditulis tetap di route; Axum mengambil nilainya melalui `Path<i32>` pada handler.
