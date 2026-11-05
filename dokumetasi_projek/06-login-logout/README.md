# 06 — Login dan Logout

Autentikasi memastikan pengguna memiliki akun dan dapat membuktikan identitas sebelum mengakses route barang. Route autentikasi didaftarkan di `src/routes/auth_routes.rs` dan diproses oleh `src/controllers/auth_controller.rs`. Model `src/models/user.rs` mengakses tabel `users` melalui pool PostgreSQL.

## Registrasi

`POST /register` menerima JSON dengan `username` dan `password`. Handler menolak username kosong atau password kosong dengan `400 Bad Request`. Jika username sudah digunakan, API merespons `409 Conflict`. Jika belum, model membuat hash Argon2 dengan salt unik dan menyimpannya. Berdasarkan default skema hasil migration, akun baru berstatus pending dan nonaktif; API mengembalikan `201 Created`.

## Login

1. Client mengirim `POST /login` dengan username dan password dalam JSON.
2. Model mengambil ID, hash password, dan status aktif terbaru dari database berdasarkan username.
3. Password diverifikasi menggunakan Argon2. Token hanya diterbitkan bila password benar dan akun aktif. Akun tidak ditemukan, password salah, dan akun nonaktif mendapat respons generik `401 Unauthorized`.
4. Jika login diizinkan, handler membuat JWT HS256 dengan claim `sub` (ID pengguna sebagai teks), `iat` (waktu diterbitkan), dan `exp` (kedaluwarsa 30 menit kemudian), lalu mengembalikannya sebagai `token` bertipe `Bearer`.

Client mengirim token pada header `Authorization: Bearer <jwt>` saat mengakses route barang. Detail validasi JWT dijelaskan di [08 — JWT HMAC](../08-jwt-hmac/README.md).

## Logout

`POST /logout` memerlukan header Bearer. Handler memvalidasi signature HS256 serta claim `sub`, `iat`, dan `exp`, lalu mengembalikan respons berhasil. Logout tidak mencabut token di server; token tetap dapat dipakai sampai kedaluwarsa. Client sebaiknya menghapus token lokal setelah logout.

Endpoint terlindungi tetap memvalidasi token dan memeriksa status akun serta role terkini dari database. Token yang dipakai untuk logout tidak langsung menjadi tidak berlaku.

Hasil pengujian runtime manual untuk login, akses barang, logout, dan penggunaan token setelah logout dicatat di [panduan JWT HMAC](../08-jwt-hmac/README.md). Pengujian token kedaluwarsa belum dilakukan secara runtime.

## Endpoint autentikasi

| Method | Path | Fungsi |
|---|---|---|
| POST | `/register` | Mendaftarkan akun |
| POST | `/login` | Memeriksa kredensial dan membuat JWT |
| POST | `/logout` | Memvalidasi JWT dan mengembalikan respons berhasil |

Route autentikasi tidak dipasang di dalam middleware route barang. Handler logout memvalidasi Bearer token sendiri; login dan register tidak memerlukan token.

