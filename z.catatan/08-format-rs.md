# 08 — `src/views/format.rs`: format harga

Fungsi `format_nominal(harga: f64) -> String` mengubah angka menjadi teks dengan pemisah ribuan titik dan desimal koma. Contoh: `1500000.0` → `Rp1.500.000,00`.

## Alur per bagian
```rust
let nominal = format!("{:.2}", harga);
```
Mengubah angka menjadi teks dengan dua angka di belakang koma. Tanpa format `:.2`, angka bisa tampil dengan jumlah digit desimal yang berbeda-beda.

```rust
let bagian: Vec<&str> = nominal.split('.').collect();
let angka = bagian[0];
let desimal = bagian[1];
```
`split('.')` membagi teks pada titik desimal, `collect()` mengumpulkan potongan ke `Vec`, indeks 0 mengambil bagian bilangan bulat, indeks 1 mengambil desimal. Ini mengasumsikan hasil `format!("{:.2}", harga)` selalu mengandung titik sebagai pemisah desimal pada format Rust ini.

```rust
let mut hasil = String::new();
for (i, digit) in angka.chars().rev().enumerate() {
    if i > 0 && i % 3 == 0 { hasil.push('.'); }
    hasil.push(digit);
}
```
- `String::new()` membuat string kosong.
- `chars()` mengambil karakter.
- `rev()` membalik urutan supaya pengelompokan dimulai dari digit paling kanan.
- `enumerate()` memberi indeks mulai dari 0.
- `i > 0 && i % 3 == 0` berarti setelah tiga digit, enam digit, dan seterusnya, tambahkan titik. `&&` berarti kedua syarat harus benar.
- `push()` menambahkan karakter ke string.

```rust
let angka_format: String = hasil.chars().rev().collect();
format!("Rp{},{}", angka_format, desimal)
```
Membalik kembali hasil agar urutan angka normal, lalu menyusun teks dengan awalan Rp dan desimal koma.

## Mengapa tidak sekadar `format!("Rp {}", harga)`?
Cara singkat itu bisa menghasilkan format seperti `Rp 1500000` atau `Rp 1500000.0`, bukan format lokal yang diinginkan. Fungsi khusus memastikan tampilan seragam.

## Alternatif
- Untuk latihan, fungsi manual ini memperlihatkan cara kerja string dan iterator.
- Untuk aplikasi keuangan yang lebih serius, gunakan library formatting locale/currency yang teruji dan tipe decimal untuk uang. `f64` dapat memiliki persoalan presisi biner.

## Catatan source saat ini
Format akhirnya `format!("Rp{},{}", angka_format, desimal)`, sehingga tidak ada spasi setelah `Rp`. Jika guru menginginkan `Rp 1.500.000,00`, ubah menjadi `format!("Rp {},{}", angka_format, desimal)`.

## Kesalahan yang perlu dihindari
Mengakses `bagian[1]` akan panic jika teks tidak punya bagian kedua. Pada fungsi ini, `format!("{:.2}", harga)` menjamin dua digit desimal dan pemisah titik dalam output Rust, tetapi jika suatu saat sumber teks berubah, pendekatan indeks perlu ditinjau ulang.
