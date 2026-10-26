# 04 — `src/models/barang.rs`

Model mendefinisikan bentuk data barang, validasi data masuk, serta fungsi untuk membaca dan mengubah tabel `barang` melalui SQLx.

## Struct data

`Barang` adalah model hasil query. `FromRow` memungkinkan SQLx memetakan kolom query ke field dengan nama yang sesuai, dan `Serialize` tersedia bila model perlu diserialisasi. Harga bertipe `rust_decimal::Decimal`, cocok untuk nilai desimal seperti uang.

`BarangBaru` adalah bentuk JSON lengkap untuk POST dan PUT. `Deserialize` memungkinkan extractor Axum membaca request JSON menjadi struct ini.

`BarangUpdate` adalah bentuk JSON parsial untuk PATCH. Field `Option<T>` dapat berisi `Some(nilai)` ketika dikirim atau `None` ketika tidak disertakan. Harga parsial juga menggunakan `Option<Decimal>`.

Contoh request:

```json
{"nama":"Monitor","stok":10,"harga":1500000.50}
```

## `BarangBaru::validasi`

Mengumpulkan pesan kesalahan dalam `Vec<String>`. Nama yang kosong setelah `trim()` ditolak; stok negatif ditolak; harga di bawah `Decimal::ZERO` ditolak. Fungsi mengembalikan `Ok(())` jika valid atau `Err(errors)` jika ada masalah. Validasi harga memakai perbandingan Decimal, tanpa konversi ke `f64`.

Validasi aplikasi memberi pesan yang jelas kepada client. Constraint database tetap dapat dipakai sebagai perlindungan tambahan.

## Query dan parameter binding

Semua query memakai placeholder PostgreSQL (`$1`, `$2`, dan seterusnya) serta `.bind(...)`. Parameter binding mengirim nilai terpisah dari teks SQL dan menghindari penyusunan query dari string input.

Kolom `harga` dibaca dan dikembalikan sebagai `NUMERIC` PostgreSQL langsung ke `Decimal`. Cast lama `harga::float8 AS harga` sudah dihilangkan karena mengubah nilai uang menjadi floating point.

## Fungsi model

- `semua(pool)`: menjalankan SELECT semua barang, mengurutkan berdasarkan `id`, lalu `fetch_all` menghasilkan `Vec<Barang>`.
- `berdasarkan_id(pool, id)`: SELECT dengan parameter ID. `fetch_optional` menghasilkan `Some(Barang)` jika ada dan `None` jika tidak ada.
- `buat(pool, data)`: INSERT nama, stok, dan harga. `RETURNING` mengambil baris yang baru dibuat; `fetch_one` dipakai karena hasil satu baris diharapkan.
- `ubah(pool, id, data)`: PUT mengganti nama, stok, dan harga. Hasil `fetch_optional` membedakan barang ditemukan atau tidak.
- `patch(pool, id, data)`: memperbarui field yang tersedia. `COALESCE(parameter, kolom)` mempertahankan nilai lama saat parameter tidak diberikan. Harga memakai Decimal dan tetap disimpan pada kolom `NUMERIC`.
- `hapus(pool, id)`: DELETE memakai `RETURNING id`; `fetch_optional` memberi tahu apakah baris terhapus. Hasil `is_some()` menjadi `true` jika ada dan `false` jika ID tidak ditemukan.

Semua fungsi database bersifat `async`, mengembalikan `Result`, dan meneruskan error SQLx dengan `?`.

## Perubahan tipe harga

Harga sebelumnya menggunakan `f64` di model dan input. Sekarang `Barang`, `BarangBaru`, dan `BarangUpdate` memakai `Decimal`. Query SELECT/RETURNING juga tidak lagi meng-cast `NUMERIC` menjadi `float8`. Format tampilan rupiah dilakukan di View, bukan di model.
