# 02 — Listener

Listener membuka alamat jaringan dan port agar aplikasi dapat menerima koneksi HTTP. Di `src/main.rs`, urutan startup dimulai dengan memuat environment melalui `dotenvy::dotenv()`, membaca `DATABASE_URL`, lalu membuat `PgPool` dengan maksimum lima koneksi. Setelah koneksi PostgreSQL berhasil, terminal mencetak `Berhasil terhubung ke PostgreSQL!`; aplikasi lalu membaca `JWT_SECRET` dan memeriksa panjang minimal 32 byte, kemudian menyusun router. Berikutnya aplikasi membaca `APP_HOST` dan `APP_PORT`, mem-parsing port sebagai `u16`, lalu mengikat `TcpListener` ke pasangan host dan port tersebut. Jika konfigurasi tidak ditemukan atau tidak valid, koneksi database gagal, secret terlalu pendek, atau alamat tidak dapat dipakai, startup berhenti dengan pesan error.

## Tujuan Pembelajaran

- Menjelaskan fungsi `TcpListener` dan `axum::serve` saat server dimulai.
- Mengidentifikasi konfigurasi host dan port yang dibutuhkan aplikasi.
- Membedakan tugas listener, server Axum, dan router dalam menerima request.

```rust
let listener = TcpListener::bind((app_host.as_str(), app_port))
    .await
    .unwrap_or_else(|error| panic!("Gagal membuka listener pada {app_host}:{app_port}: {error}"));
axum::serve(listener, app).await.expect("Server gagal berjalan");
```

`TcpListener::bind` menyiapkan socket yang menunggu koneksi. `axum::serve` menjalankan layanan Axum menggunakan listener dan router `app`. Router menentukan handler yang cocok setelah request diterima. Jadi ketiganya punya tugas berbeda: listener menerima koneksi, server Axum mengelola request/response, dan router memilih jalur pemrosesan.

Jika port sudah dipakai atau alamat tidak dapat diikat, proses bind gagal dan server tidak mulai melayani request. Setelah bind berhasil, aplikasi membaca alamat aktual listener dan mencetak `Server berjalan di http://{address}`. `127.0.0.1` membatasi akses ke komputer lokal; alamat bind yang dapat dijangkau perangkat lain perlu disesuaikan dengan lingkungan deployment.

Jalankan backend dari root repository dengan Cargo:

```bash
cargo run --manifest-path backend/Cargo.toml
```

Nilai contoh konfigurasi tersedia di `.env.example`. Startup hanya mencapai tahap listener setelah koneksi PostgreSQL berhasil, tetapi `GET /health` sendiri merespons JSON sederhana tanpa menjalankan query. Karena itu, health check memastikan server HTTP merespons, bukan bahwa koneksi database masih sehat atau seluruh operasi database berhasil. Aplikasi tidak membuat atau mengubah tabel saat startup.
