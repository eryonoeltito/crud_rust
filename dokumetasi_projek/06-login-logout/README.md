# 06 — Login dan Logout

Autentikasi memastikan pengguna memiliki akun dan dapat membuktikan identitas sebelum mengakses route barang. Route autentikasi didaftarkan di `src/routes/auth_routes.rs` dan diproses oleh `src/controllers/auth_controller.rs`. Model `src/models/user.rs` mengakses tabel `users` melalui pool PostgreSQL.

## Registrasi

`POST /register` menerima JSON dengan `username` dan `password`. Handler menolak username kosong atau password kosong dengan `400 Bad Request`. Jika username sudah digunakan, API merespons `409 Conflict`. Jika belum, model membuat hash Argon2 dengan salt unik dan menyimpannya. Berdasarkan default skema hasil migration, akun baru berstatus pending dan nonaktif; API mengembalikan `201 Created`.

## Login

1. Client mengirim `POST /login` dengan username dan password dalam JSON.
2. Model mengambil ID, hash password, dan status aktif terbaru dari database berdasarkan username.
3. Password diverifikasi menggunakan Argon2. Token hanya diterbitkan bila password benar dan akun aktif. Akun tidak ditemukan, password salah, dan akun nonaktif mendapat respons generik `401 Unauthorized`.
4. Jika login diizinkan, handler membuat JWT HS256 dengan claim `sub` (ID pengguna sebagai teks), `exp` (kedaluwarsa satu jam), dan `jti` acak, lalu mengembalikannya sebagai `token` bertipe `Bearer`.

Client mengirim token pada header `Authorization: Bearer <jwt>` saat mengakses route barang. Detail validasi JWT dijelaskan di [08 — JWT HMAC](../08-jwt-hmac/README.md).

## Logout

`POST /logout` memerlukan header Bearer. Handler memvalidasi signature HS256, claim `sub`, `exp`, dan `jti`, lalu menyimpan `jti` serta waktu kedaluwarsa dari claim ke `public.revoked_tokens`. Insert memakai `ON CONFLICT (jti) DO NOTHING`, sehingga logout berulang tetap berhasil tanpa membuat baris duplikat. Token lama tanpa `jti` ditolak; pengguna perlu login kembali.

Middleware route barang saat ini belum memeriksa `public.revoked_tokens`. Karena itu, logout sudah mencatat pencabutan di server, tetapi belum membuat middleware endpoint barang menolak token tersebut. Penolakan pada endpoint terlindungi akan diterapkan pada Tahap 2.

Hasil pengujian runtime manual untuk login, akses barang, logout, dan penggunaan token setelah logout dicatat di [panduan JWT HMAC](../08-jwt-hmac/README.md). Pengujian token kedaluwarsa belum dilakukan secara runtime.

## Endpoint autentikasi

| Method | Path | Fungsi |
|---|---|---|
| POST | `/register` | Mendaftarkan akun |
| POST | `/login` | Memeriksa kredensial dan membuat JWT |
| POST | `/logout` | Memvalidasi JWT dan mencatat pencabutan token |

Route autentikasi tidak dipasang di dalam middleware revocation route barang. Handler logout memvalidasi Bearer token sendiri agar token yang sudah dicabut tetap dapat dipakai untuk mengulang logout selama belum kedaluwarsa. Login tidak memerlukan token.

