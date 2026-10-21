# 10 — Ringkasan CRUD dan cara menguji

| Tujuan | Request | Body contoh | Hasil yang diharapkan |
|---|---|---|---|
| Cek server | `GET /health` | — | JSON status server |
| Daftar | `GET /barang` | — | Array berisi banyak barang |
| Detail | `GET /barang/1` | — | Array satu barang atau 404 |
| Tambah | `POST /barang` | `{"nama":"Monitor","stok":10,"harga":1500000}` | 201 Created dan data baru |
| Ganti lengkap | `PUT /barang/1` | `{"nama":"Monitor","stok":12,"harga":1600000}` | Data terbaru atau 404 |
| Ubah sebagian | `PATCH /barang/1` | `{"stok":15}` | Hanya stok berubah atau 404 |
| Hapus | `DELETE /barang/1` | — | Pesan berhasil atau 404 |

Ganti port/alamat sesuai tempat server berjalan. Pada source ini server bind ke `127.0.0.1:3000`.

"Request masuk lewat route, route memilih controller. Controller membaca input dan memvalidasi, lalu memanggil model untuk query database. Hasil query ditangani controller, disusun menjadi JSON oleh view bila digunakan, kemudian dikirim sebagai respons dengan status HTTP yang sesuai."

