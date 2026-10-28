# Panduan Catatan Kode Inventaris

Catatan ini mengikuti source proyek yang ada saat ini. Fokusnya bukan cuma arti fungsi, tetapi **alasan digunakan, apa yang terjadi jika tidak digunakan, dan contoh alternatif**.

## Isi catatan
- `01-alur-aplikasi.md` — alur request dan pembagian MVC.
- `02-cargo-toml.md` — dependencies dan alasan pemilihan.
- `03-main-rs.md` — menjalankan server, pool database, state, router, port.
- `17-database-rs.md` — inisialisasi tabel `barang` dan `users` saat startup.
- `04-barang-rs.md` — model, validasi, query SQLx, parameter binding.
- `05-barang-controller-rs.md` — handler, validasi, status HTTP, error.
- `06-barang-routes-rs.md` — endpoint dan HTTP method.
- `07-barang-view-rs.md` — bentuk JSON respons.
- `08-format-rs.md` — format harga rupiah.
- `09-gitignore.md` — melindungi file lokal seperti `.env` dari Git.
- `10-ringkasan-crud.md` — tabel endpoint dan alur.

**File `mod.rs` sengaja tidak dibuatkan catatan**, sesuai permintaan. File itu tetap disebut seperlunya bila dibutuhkan untuk memahami cara modul dipanggil.

> Batasan: catatan menjelaskan source aplikasi dan tidak menyertakan isi `.env`, database, atau frontend. Jangan pernah mengirim password database asli.

Dokumentasi autentikasi yang dipisah per file:
- `11-model-user.md` — penjelasan kode `models/user.rs`.
- `12-controller-auth.md` — penjelasan kode controller login/logout.
- `13-routes-auth.md` — penjelasan route login/logout.
- `14-middleware-auth.md` — penjelasan middleware pemeriksaan session.
- `15-main-session.md` — penjelasan konfigurasi session pada `main.rs`.
- `16-tes-curl-session.md` — pengujian cURL terpisah dari penjelasan kode.
