# 07 — `src/views/barang_view.rs`

View menyusun data model menjadi bentuk JSON untuk client. Respons saat ini memakai objek dengan nama field, dan harga dikirim sebagai teks nominal rupiah.

## `BarangResponse`

Struct response memiliki field `id`, `nama`, `stok`, dan `harga`. `Serialize` membuat struct dapat diubah Axum menjadi JSON. Field `harga` bertipe `String` karena nilainya sudah diformat, misalnya `Rp1.500.000,00`.

Contoh satu barang:

```json
{"id":1,"nama":"Monitor","stok":10,"harga":"Rp1.500.000,00"}
```

## `From<Barang> for BarangResponse`

Implementasi `From` mengubah satu model `Barang` menjadi DTO response. ID, nama, dan stok dipindahkan ke field yang namanya sama. Harga yang di model bertipe `Decimal` dilewatkan ke `format_nominal`, sehingga hanya representasi output yang menjadi teks rupiah.

Pola `From` membuat konversi dapat dipakai kembali oleh response daftar, detail, POST, PUT, dan PATCH.

## `list_barang_response`

Fungsi menerima `Vec<Barang>`, mengonsumsi isinya dengan `into_iter()`, mengubah setiap item melalui `BarangResponse::from`, lalu mengumpulkan hasil menjadi `Vec<BarangResponse>`. `Json(data)` membungkus koleksi tersebut sebagai response JSON Axum.

Contoh hasil:

```json
[
  {"id":1,"nama":"Monitor","stok":10,"harga":"Rp1.500.000,00"},
  {"id":2,"nama":"Keyboard","stok":20,"harga":"Rp250.000,00"}
]
```

## `detail_barang_response`

Fungsi menerima satu `Barang`, mengubahnya menjadi `BarangResponse`, lalu membungkusnya dengan `Json`. Fungsi yang sama dipakai oleh detail GET, POST, PUT, dan PATCH supaya semua respons satu barang memiliki bentuk konsisten.

## Perubahan dari versi sebelumnya

- Respons sebelumnya berupa array posisi seperti `[1,"Monitor",10,"Rp1.500.000,00"]`; bentuk itu telah dihilangkan.
- Data tidak lagi dirakit dengan `serde_json::Value` dan `json!`. DTO terstruktur sekarang menjelaskan nama dan tipe setiap field.
- Harga tetap diformat melalui `format_nominal`, namun sumber nilainya sekarang `Decimal`.
- Contoh object yang dulu hanya alternatif sekarang merupakan kontrak respons aktif.
