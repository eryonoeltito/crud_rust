# 00 — Koneksi Database

Bagian ini menjelaskan cara aplikasi terhubung ke PostgreSQL dan menyiapkan tabel yang digunakan. PostgreSQL adalah tempat data barang dan akun disimpan agar tetap tersedia setelah aplikasi berhenti. Koneksi database disiapkan sekali saat startup dan digunakan kembali oleh handler.

## Konfigurasi dan pool

Di `src/main.rs`, `dotenvy::dotenv()` mencoba memuat variabel dari `.env`, lalu `std::env::var("DATABASE_URL")` mengambil alamat koneksi. URL umumnya memuat alamat server, nama database, dan kredensial. Jika nilainya tidak tersedia, `expect` menghentikan startup dengan pesan yang jelas.

`PgPoolOptions` membuat pool PostgreSQL. Pool mengelola beberapa koneksi yang dapat dipakai ulang oleh request, sehingga aplikasi tidak perlu membuat koneksi baru setiap kali handler dijalankan. Batas `max_connections(5)` membatasi koneksi database serentak, bukan jumlah pengguna API. Jika seluruh koneksi sedang dipakai, operasi berikutnya menunggu sampai satu koneksi tersedia.

Simpan URL dan kredensial di environment, bukan di source atau dokumentasi. `.env.example` berisi contoh nama konfigurasi tanpa perlu mencantumkan rahasia.

## Inisialisasi schema

Setelah pool tersambung, `database::initialize(&pool)` di `src/database.rs` membuat tabel `barang` dan `users` jika belum tersedia. Tabel `barang` menyimpan `id`, `nama`, `stok`, dan `harga`; ID dibuat oleh PostgreSQL dan harga menggunakan tipe `NUMERIC`. Tabel `users` memakai username sebagai primary key serta menyimpan password sesuai model autentikasi saat ini.

`CREATE TABLE IF NOT EXISTS` hanya membuat tabel yang belum ada. Perintah tersebut tidak memperbaiki atau menyelaraskan kolom pada tabel lama. Jika inisialisasi gagal, aplikasi berhenti sebelum server menerima request. Dengan demikian, server tidak mulai melayani API dalam keadaan schema yang diharapkan belum siap.

> Catatan keamanan: implementasi model pengguna saat ini menyimpan dan membandingkan password secara langsung sebagai teks. Untuk penggunaan nyata, password sebaiknya disimpan sebagai hash yang dibuat dengan algoritma khusus password, bukan sebagai teks biasa.

## Alur singkat

```text
DATABASE_URL → PgPoolOptions → PgPool bersama → database::initialize → Router/handler
```

Pool yang sama diberikan ke router melalui `.with_state(pool)` dan diambil oleh handler dengan `State<PgPool>`. Handler kemudian meneruskannya ke model yang menjalankan query. Cara request dipetakan ke handler dijelaskan di [01 — Routes dan Handler](../01-routes-handler/README.md), sedangkan rincian query barang ada di [03 — CRUD Barang](../03-crud-barang/README.md).
