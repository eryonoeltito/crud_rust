# 05 — Objek JSON berupa JSON Array

JSON (JavaScript Object Notation) adalah format teks yang umum digunakan client dan server untuk bertukar data. Endpoint daftar barang mengembalikan JSON array karena hasilnya dapat berisi nol, satu, atau banyak barang. Setiap elemen array adalah objek JSON dengan field bernama, seperti `id`, `nama`, `stok`, dan `harga`.

Contoh bentuk response:

```json
[
  {"id": 1, "nama": "Monitor", "stok": 10, "harga": "Rp1.500.000,50"},
  {"id": 2, "nama": "Mouse", "stok": 25, "harga": "Rp75.000,00"}
]
```

Kurung siku `[` dan `]` menandai array, sedangkan kurung kurawal `{` dan `}` menandai satu objek. Array cocok untuk menyatakan kumpulan barang: tiap objek dapat dibaca berdasarkan nama field, dan jumlah elemen dapat berubah sesuai isi database. Jika belum ada data, bentuk daftar yang wajar adalah `[]` (array kosong), bukan `null`, karena endpoint tetap mengembalikan jenis data daftar.

Pada proyek ini, model mengambil daftar menjadi `Vec<Barang>` menggunakan `fetch_all`. Controller menyerahkan hasil tersebut ke `list_barang_response`, yang mengubah setiap model ke `BarangResponse`, termasuk memformat harga. Axum kemudian menserialisasi `Vec` tersebut menjadi JSON array. Karena tiap item berupa objek bernama, client dapat mengambil `item.nama` atau `item.harga` tanpa bergantung pada urutan properti. Endpoint detail mengembalikan satu objek, sedangkan endpoint daftar mengembalikan array objek.

Contoh sederhana membaca response di JavaScript:

```javascript
const barang = await response.json();
for (const item of barang) {
  console.log(item.nama, item.stok);
}
```

Client harus tetap menangani kemungkinan response error, misalnya status `401` jika session tidak ada atau `500` jika terjadi kegagalan server. Rincian status dan operasi endpoint dapat dibaca di [03 — CRUD Barang](../03-crud-barang/README.md).
