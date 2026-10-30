# 02 — Listener

Listener membuka alamat jaringan dan port agar aplikasi dapat menerima koneksi HTTP. Di `src/main.rs`, aplikasi membaca `HOST` dan `PORT` dari environment setelah pemanggilan `dotenv()`, lalu mengikat `TcpListener` ke alamat tersebut. Port diparsing sebagai `u16`, yaitu bilangan bulat yang sesuai untuk nomor port. Jika variabel tidak ditemukan, port bukan angka, atau alamat tidak dapat dipakai, startup berhenti dengan pesan error.

```rust
let listener = TcpListener::bind((app_host.as_str(), app_port)).await?;
axum::serve(listener, app).await?;
```

`TcpListener::bind` menyiapkan socket yang menunggu koneksi. `axum::serve` menjalankan layanan Axum menggunakan listener dan router `app`. Router menentukan handler yang cocok setelah request diterima. Jadi ketiganya punya tugas berbeda: listener menerima koneksi, server Axum mengelola request/response, dan router memilih jalur pemrosesan.

Jika port sudah dipakai atau alamat tidak dapat diikat, proses bind gagal dan server tidak mulai melayani request. `127.0.0.1` membatasi akses ke komputer lokal; alamat bind yang dapat dijangkau perangkat lain perlu disesuaikan dengan lingkungan deployment. Aplikasi mencetak alamat yang berhasil dibuka agar developer tahu URL dasar untuk mengirim request.

Nilai contoh konfigurasi tersedia di `.env.example`. Setelah server berjalan, endpoint `GET /health` merespons JSON sederhana dengan status sukses. Ini memastikan server HTTP merespons, tetapi tidak menjalankan query dan karena itu tidak memastikan seluruh operasi database berhasil. Proses startup sendiri sudah melakukan koneksi dan inisialisasi tabel sebelum membuka listener.
