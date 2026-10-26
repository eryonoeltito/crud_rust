# 10 — Ringkasan CRUD dan cara menguji

Semua endpoint `/barang` membutuhkan session login. Body POST dan PUT mengirim `harga` sebagai angka desimal JSON; model memakai `Decimal` untuk menjaga nilai uang. Field `harga` pada respons berupa string dengan format rupiah Indonesia.

| Tujuan | Request | Body contoh | Hasil yang diharapkan |
|---|---|---|---|
| Cek server | `GET /health` | — | JSON status server |
| Daftar | `GET /barang` | — | Array objek barang bernama |
| Detail | `GET /barang/1` | — | Objek barang atau 404 |
| Tambah | `POST /barang` | `{"nama":"Monitor","stok":10,"harga":1500000.50}` | 201 Created dan objek barang |
| Ganti lengkap | `PUT /barang/1` | `{"nama":"Monitor","stok":12,"harga":1600000}` | Objek terbaru atau 404 |
| Ubah sebagian | `PATCH /barang/1` | `{"stok":15}` | Objek terbaru atau 404 |
| Hapus | `DELETE /barang/1` | — | Pesan berhasil atau 404 |

Contoh bentuk respons barang:

```json
{
  "id": 1,
  "nama": "Monitor",
  "stok": 10,
  "harga": "Rp1.500.000,50"
}
```

PATCH harus memuat setidaknya satu field dari `nama`, `stok`, atau `harga`. Body `{}` mendapatkan `400 Bad Request`. Field yang tidak disertakan pada PATCH tetap memakai nilai lama.

Ganti port/alamat sesuai tempat server berjalan. Pada source ini server bind ke `127.0.0.1:3000`.

"Request masuk lewat route, route memilih controller. Controller membaca input dan memvalidasi, lalu memanggil model untuk query database. Hasil query ditangani controller, disusun menjadi objek JSON oleh view bila digunakan, kemudian dikirim sebagai respons dengan status HTTP yang sesuai."
