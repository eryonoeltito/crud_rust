# 00 — Koneksi Database

Bagian ini menjelaskan cara aplikasi terhubung ke PostgreSQL. PostgreSQL adalah tempat data barang dan akun disimpan agar tetap tersedia setelah aplikasi berhenti. Koneksi database disiapkan sekali saat startup dan digunakan kembali oleh handler.

## Tujuan Pembelajaran

- Menjelaskan fungsi `DATABASE_URL` dan pool koneksi PostgreSQL.
- Menjelaskan persiapan skema database yang diperlukan sebelum API digunakan.
- Membedakan penyimpanan password berbentuk hash Argon2 dari teks biasa.

## Konfigurasi dan pool

Di `src/main.rs`, `dotenvy::dotenv()` mencoba memuat variabel dari `.env`, lalu `std::env::var("DATABASE_URL")` mengambil alamat koneksi. URL umumnya memuat alamat server, nama database, dan kredensial. Jika nilainya tidak tersedia, `expect` menghentikan startup dengan pesan yang jelas.

`PgPoolOptions` membuat pool PostgreSQL. Pool mengelola beberapa koneksi yang dapat dipakai ulang oleh request, sehingga aplikasi tidak perlu membuat koneksi baru setiap kali handler dijalankan. Batas `max_connections(5)` membatasi koneksi database serentak, bukan jumlah pengguna API. Jika seluruh koneksi sedang dipakai, operasi berikutnya menunggu sampai satu koneksi tersedia.

Simpan URL dan kredensial di environment, bukan di source atau dokumentasi. `.env.example` berisi contoh nama konfigurasi tanpa perlu mencantumkan rahasia.

## Skema database

Saat startup, aplikasi hanya membuka koneksi PostgreSQL dan menyiapkan `PgPool`. Aplikasi tidak membuat atau mengubah tabel. Skema yang dibutuhkan harus sudah tersedia di database sebelum endpoint yang mengaksesnya digunakan.

Tabel `barang` menyimpan `id`, `nama`, `stok`, dan `harga`; ID dibuat oleh PostgreSQL dan harga menggunakan tipe `NUMERIC`. Tabel `users` memakai username sebagai primary key serta menyimpan password sesuai model autentikasi saat ini. Penyiapan struktur tabel dilakukan secara terpisah dari proses menjalankan aplikasi.

Password pengguna disimpan sebagai hash Argon2, bukan sebagai teks biasa. Akun lama yang masih berisi teks biasa tidak dapat login setelah perubahan ini. Sebelum pengguna lama mendaftar ulang dengan username yang sama, administrator perlu menghapus baris akun lama dari tabel `users`; aplikasi tidak menghapus data pengguna secara otomatis.

## Alur singkat

```text
DATABASE_URL → PgPoolOptions → PgPool bersama → Router/handler
```

Pool yang sama diberikan ke router melalui `.with_state(pool)` dan diambil oleh handler dengan `State<PgPool>`. Handler kemudian meneruskannya ke model yang menjalankan query. Cara request dipetakan ke handler dijelaskan di [01 — Routes dan Handler](../01-routes-handler/README.md), sedangkan rincian query barang ada di [03 — CRUD Barang](../03-crud-barang/README.md).
