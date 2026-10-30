# 04 — Format Harga

Harga adalah nilai yang perlu diperlakukan hati-hati karena representasi bilangan pecahan komputer dapat memiliki selisih kecil. Proyek ini menyimpan harga sebagai angka desimal di database, lalu memformatnya untuk tampilan response oleh `src/views/format.rs`. Pemisahan ini menjaga nilai database tetap numerik sementara API dapat menyajikan nominal yang mudah dibaca.

## Tipe dan alur data

Model barang menggunakan `rust_decimal::Decimal`, yang dipetakan SQLx ke kolom PostgreSQL `NUMERIC`. Query tidak mengubah harga menjadi `float8`/`f64`. Tipe decimal dirancang untuk menyimpan digit desimal secara tepat, sehingga sesuai untuk nilai uang dan validasi seperti harga tidak boleh kurang dari nol.

View mengubah model `Barang` menjadi `BarangResponse`. Saat konversi itu dilakukan, fungsi `format_nominal` membuat string rupiah: dua digit desimal dipertahankan, pemisah ribuan menggunakan titik, dan pemisah desimal menggunakan koma. Contohnya, nilai `1500000.50` menjadi `Rp1.500.000,50`. Pada response barang, harga menjadi string yang sudah diformat. Untuk request create/update, client mengirim harga sebagai angka JSON.

```text
JSON angka → Decimal → PostgreSQL NUMERIC
PostgreSQL NUMERIC → Decimal → format rupiah → string JSON
```

Dengan memisahkan pemformatan dari query, aturan tampilan dapat diubah tanpa mengubah cara database menyimpan harga. Implementasinya ada di `src/views/format.rs` dan penyusunan respons barang ada di `src/views/barang_view.rs`. Contoh response lengkap tersedia di [05 — Objek JSON berupa JSON Array](../05-json-array/README.md).
