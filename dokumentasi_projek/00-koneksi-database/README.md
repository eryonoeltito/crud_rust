# 00 — Koneksi Database

Aplikasi memakai PostgreSQL melalui SQLx. Saat startup, `main.rs` membaca `DATABASE_URL`, membuat `PgPool` dengan maksimum lima koneksi, lalu menghentikan proses jika koneksi gagal.

## Pool dan koneksi

`PgPool` adalah pengelola koneksi yang dapat dipakai bersama oleh banyak request. Handler menerima pool melalui `State<PgPool>`. Middleware menerima pool melalui `Extension<PgPool>`, kemudian memanggil `pool.acquire()` ketika perlu menjalankan pemeriksaan akun.

Ada dua bentuk parameter database di model:

- `&PgPool`: model barang dan riwayat membiarkan SQLx mengambil koneksi dari pool saat query dijalankan.
- `&mut PgConnection`: model login dan middleware memakai koneksi yang sudah diambil dengan `pool.acquire()`.

Setelah fungsi selesai, koneksi dikembalikan ke pool.

## Tabel yang digunakan kode

Kode menggunakan tabel `users`, `gudang`, `barang`, dan `riwayat_stok`.

| Tabel | Data yang dipakai aplikasi |
|---|---|
| `users` | `id`, `username`, `password`, `aktif`, `role`, `gudang_id` |
| `gudang` | Pemeriksaan apakah gudang dengan ID tertentu ada |
| `barang` | `id`, `nama`, `stok`, `harga`, `gudang_id` |
| `riwayat_stok` | `id`, `barang_id`, `nama_barang`, `gudang_id`, `stok_lama`, `stok_baru`, `waktu_perubahan` |

Model menggunakan `NUMERIC` PostgreSQL untuk `harga` dan memetakannya ke `rust_decimal::Decimal`. Model membaca `waktu_perubahan` sebagai teks melalui cast SQL.

Repository yang diperiksa tidak berisi file migration. Karena itu, dokumentasi ini tidak mengarang primary key, default, foreign key, data awal, atau aturan penghapusan tabel. Schema harus disiapkan terpisah sebelum endpoint database dipakai. Startup aplikasi hanya membuka koneksi; aplikasi tidak menjalankan migration.

## Alur

```text
DATABASE_URL → PgPoolOptions → PgPool
                         ├→ State<PgPool> → controller → model → PostgreSQL
                         └→ Extension<PgPool> → middleware → PostgreSQL
```

Secret dan URL database harus disimpan sebagai environment variable. Jangan menaruh kredensial nyata di dokumentasi atau source code.
