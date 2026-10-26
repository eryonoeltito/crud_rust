# 08 — `src/views/format.rs`: format harga

Fungsi `format_nominal(harga: Decimal) -> String` mengubah nilai harga bertipe `rust_decimal::Decimal` menjadi teks rupiah. Contoh: `1500000` menjadi `Rp1.500.000,00` dan `1500.5` menjadi `Rp1.500,50`.

## Alur fungsi

```rust
let nominal = format!("{harga:.2}");
```

Memformat `Decimal` dengan tepat dua angka desimal. Tipe `Decimal` menjaga representasi desimal uang tanpa memakai pendekatan floating point `f64`.

```rust
let (angka, desimal) = nominal.split_once('.').expect(...);
```

Memisahkan bagian bilangan bulat dan pecahan. Format `:.2` menjamin dua digit pecahan, sehingga hasil memiliki titik pemisah pada teks format Rust.

Fungsi kemudian memisahkan tanda minus bila ada, membalik digit bilangan bulat, dan menambahkan titik setiap tiga digit. Setelah digit dibalik kembali, fungsi menyusun hasil akhir:

```rust
format!("{tanda}Rp{angka_format},{desimal}")
```

Titik dipakai sebagai pemisah ribuan dan koma sebagai pemisah desimal sesuai format rupiah yang digunakan aplikasi. Nilai negatif juga tetap menampilkan tanda minus sebelum `Rp`.

## Di mana fungsi dipakai?

`BarangResponse::from` di `src/views/barang_view.rs` memanggil `format_nominal(item.harga)`. Pemformatan hanya terjadi pada lapisan response; model dan query database tetap memakai `Decimal`.

Karena field `harga` pada response bertipe `String`, JSON berisi teks, misalnya:

```json
{"id":1,"nama":"Monitor","stok":10,"harga":"Rp1.500.000,00"}
```

## Perubahan dari versi sebelumnya

- Parameter sebelumnya bertipe `f64`; sekarang `Decimal` agar sesuai dengan model harga dan menghindari floating point untuk uang.
- Format sebelumnya mengasumsikan angka selalu nonnegatif; versi sekarang juga mempertahankan tanda minus.
- Pemisahan memakai `split_once` alih-alih membuat `Vec` dan mengakses indeks `bagian[0]` serta `bagian[1]`.
- Contoh DTO objek JSON yang sebelumnya hanya disebut sebagai alternatif kini menjadi bentuk respons yang digunakan.
