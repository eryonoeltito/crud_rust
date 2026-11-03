# 06 — Login dan Logout

Autentikasi memastikan pengguna memiliki akun dan dapat membuktikan identitas sebelum mengakses route barang. Route autentikasi didaftarkan di `src/routes/auth_routes.rs` dan diproses oleh `src/controllers/auth_controller.rs`. Model `src/models/user.rs` mengakses tabel `users` melalui pool PostgreSQL.

## Registrasi

`POST /register` menerima JSON dengan `username` dan `password`. Handler menolak username kosong atau password kosong dengan `400 Bad Request`. Jika username sudah digunakan, API merespons `409 Conflict`. Jika belum, model membuat hash Argon2 dengan salt unik dan menyimpannya; API kemudian mengembalikan `201 Created`.

## Login

1. Client mengirim `POST /login` dengan username dan password dalam JSON.
2. Handler mengambil hash password akun dan memverifikasi password menggunakan Argon2.
3. Jika kredensial tidak cocok, API mengembalikan `401 Unauthorized`.
4. Jika cocok, API membuat JWT HS256 dengan claim `sub` (username) dan `exp` (kedaluwarsa satu jam), lalu mengembalikannya sebagai `token` bertipe `Bearer`.

Client mengirim token pada header `Authorization: Bearer <jwt>` saat mengakses route barang. Detail validasi JWT dijelaskan di [08 — JWT HMAC](../08-jwt-hmac/README.md).

## Logout

`POST /logout` mengembalikan respons logout berhasil dan meminta client menghapus token dari penyimpanan lokalnya. Server tidak membatalkan token. Token yang sama tetap dapat digunakan selama signature HS256 valid dan waktu `exp` belum terlewati. Middleware memeriksa token sebelum mengizinkan akses ke route barang; request tanpa token atau dengan token tidak valid ditolak.

Hasil pengujian runtime manual untuk login, akses barang, logout, dan penggunaan token setelah logout dicatat di [panduan JWT HMAC](../08-jwt-hmac/README.md). Pengujian token kedaluwarsa belum dilakukan secara runtime.

## Endpoint autentikasi

| Method | Path | Fungsi |
|---|---|---|
| POST | `/register` | Mendaftarkan akun |
| POST | `/login` | Memeriksa kredensial dan membuat JWT |
| POST | `/logout` | Memberi tahu client untuk membuang JWT |

Route autentikasi tidak dipasang di dalam lapisan perlindungan route barang. Dengan demikian, login dan logout tidak memerlukan token; seluruh route barang tetap dilindungi middleware.

