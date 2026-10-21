# 07 — `src/views/barang_view.rs`

View di source ini bertugas menyusun respons JSON agar format data barang konsisten.

## Daftar barang
`list_barang_response` menerima `Vec<Barang>`, lalu:
1. `into_iter()` mengonsumsi vector dan membuat iterator item.
2. `map(|item| ...)` mengubah setiap `Barang` menjadi vector berisi ID, nama, stok, dan harga yang sudah diformat.
3. `serde_json::json!(...)` mengubah setiap nilai menjadi nilai JSON.
4. `collect()` mengumpulkan hasil pemetaan menjadi satu vector luar.
5. `Json(data)` membungkus data sebagai respons JSON Axum.

Bentuknya adalah array dari array, misalnya:
```json
[[1,"Monitor",10,"Rp1.500.000,00"],[2,"Keyboard",20,"Rp250.000,00"]]
```

`collect()` dibutuhkan karena `map()` menghasilkan iterator, bukan langsung `Vec`. Tanpa `collect()`, tipe data tidak cocok dengan `Json<Vec<Vec<Value>>>` yang diminta fungsi.

## Detail barang
`detail_barang_response` menerima satu `Barang`, jadi cukup membuat satu `vec!`; tidak perlu `map()` dan `collect()` karena tidak memproses kumpulan item.

## Kenapa `serde_json::Value`?
Ini memudahkan membuat array campuran tipe—angka, teks, angka, teks—sebagai JSON. Alternatif yang lebih terstruktur adalah membuat DTO/struct response dan men-derive `Serialize`. Itu memberi nama field seperti `id`, `nama`, `stok`, `harga`, tetapi bentuk JSON akan menjadi object, bukan array posisi.

Contoh object alternatif:
```json
{"id":1,"nama":"Monitor","stok":10,"harga":"Rp1.500.000,00"}
```
Jangan mengganti bentuk respons tanpa memastikan format array memang persyaratan guru/client.

## Perbedaan penting
`Json(...)` membuat respons JSON; `json!(...)` membuat nilai JSON di dalamnya. Keduanya bukan hal yang sama.
