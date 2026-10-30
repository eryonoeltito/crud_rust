# 06 — Login dan Logout

Autentikasi memastikan pengguna memiliki akun dan dapat membuktikan identitas sebelum mengakses route barang. Route autentikasi didaftarkan di `src/routes/auth_routes.rs` dan diproses oleh `src/controllers/auth_controller.rs`. Model `src/models/user.rs` mengakses tabel `users` melalui pool PostgreSQL.

## Registrasi

`POST /register` menerima JSON dengan `username` dan `password`. Handler menolak username kosong atau password kosong dengan `400 Bad Request`. Model memeriksa apakah username telah dipakai; jika sudah, API merespons `409 Conflict`. Jika belum, akun disimpan dan API mengembalikan `201 Created`.

## Alur login

1. Client mengirim `POST /login` dengan JSON username dan password.
2. Handler meminta model memeriksa kredensial.
3. Jika tidak cocok, API mengembalikan `401 Unauthorized`.
4. Jika cocok, handler menyimpan username ke session dan mengembalikan respons berhasil. Session layer mengelola penyimpanan session serta mengirim cookie pengenal session kepada client.

## Alur logout

Client mengirim `POST /logout` dengan cookie session. Handler menghapus data session melalui `session.flush()` lalu mengembalikan respons logout. Request berikutnya ke route barang tidak lagi memiliki session aktif dan ditolak middleware. Cookie yang masih tersimpan di client tidak lagi menunjuk pada session aktif.

## Endpoint autentikasi

| Method | Path | Fungsi |
|---|---|---|
| POST | `/register` | Mendaftarkan akun |
| POST | `/login` | Memeriksa kredensial dan membuat session |
| POST | `/logout` | Mengakhiri session |

Route autentikasi tidak dipasang di dalam lapisan perlindungan route barang, agar pengguna yang belum login tetap dapat mengakses endpoint login. Cara session dan middleware bekerja dijelaskan pada [07 — Session, Cookie, dan Middleware](../07-session-cookie-middleware/README.md).

> Catatan keamanan: model `User` pada source saat ini membandingkan password secara langsung dengan teks yang tersimpan. Ini cocok hanya sebagai implementasi pembelajaran; aplikasi nyata perlu menyimpan hash password dengan algoritma khusus password dan tidak mencatat kredensial ke log.
