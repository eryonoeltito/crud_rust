# Panduan Catatan Kode Inventaris

Catatan ini mengikuti source dalam `inventaris_api_aman(1).zip` yang diperiksa 20 Oktober 2026. Fokusnya bukan cuma arti fungsi, tetapi **alasan digunakan, apa yang terjadi jika tidak digunakan, dan contoh alternatif**.

## Isi catatan
- `01-alur-aplikasi.md` — alur request dan pembagian MVC.
- `02-cargo-toml.md` — dependencies dan alasan pemilihan.
- `03-main-rs.md` — menjalankan server, pool database, state, router, port.
- `04-barang-rs.md` — model, validasi, query SQLx, parameter binding.
- `05-barang-controller-rs.md` — handler, validasi, status HTTP, error.
- `06-barang-routes-rs.md` — endpoint dan HTTP method.
- `07-barang-view-rs.md` — bentuk JSON respons.
- `08-format-rs.md` — format harga rupiah.
- `09-gitignore.md` — melindungi file lokal seperti `.env` dari Git.
- `10-ringkasan-crud.md` — tabel endpoint dan alur.
- `11-latihan-pkl.md` — pertanyaan latihan beserta arah jawaban.

**File `mod.rs` sengaja tidak dibuatkan catatan**, sesuai permintaan. File itu tetap disebut seperlunya bila dibutuhkan untuk memahami cara modul dipanggil.

> Batasan: ZIP ini tidak menyertakan `.env`, database, atau frontend. Catatan menjelaskan source yang ada di ZIP, bukan memastikan aplikasi sudah diuji runtime. Jangan pernah mengirim password database asli.
