# 07 — JWT HMAC dan Middleware

API menggunakan JWT bertanda tangan HMAC SHA-256 (HS256) sebagai token autentikasi. Token membawa ID pengguna sebagai teks pada claim `sub`, waktu diterbitkan pada claim `iat`, dan waktu kedaluwarsa pada claim `exp`. Middleware memverifikasi signature dan masa berlaku token sebelum meneruskan request ke route terlindungi.

## Konfigurasi secret

Secret JWT dibaca dari environment variable `JWT_SECRET`. Nilainya harus rahasia dan minimal 32 byte. Simpan di `.env` atau pengelola secret deployment; jangan commit nilai secret yang sebenarnya. Untuk membuat secret acak, jalankan `openssl rand -hex 32`, lalu masukkan hasilnya ke `.env` sebagai `JWT_SECRET`.

## Siklus request

```text
Login berhasil → server mengirim JWT HS256 di response JSON
Request berikutnya + Authorization: Bearer <token> → middleware memverifikasi token
```

Token berlaku selama 30 menit. Kirim token pada header `Authorization` untuk setiap request yang memerlukan login:

```http
Authorization: Bearer <token>
```

`wajib_login` berada di `src/middleware/auth_middleware.rs` dan melindungi route yang memerlukannya. Token yang hilang, rusak, memiliki signature salah, atau kedaluwarsa ditolak dengan `401 Unauthorized`. Setelah validasi JWT, middleware mengambil status akun, role, dan gudang terbaru dari database. `POST /logout` memvalidasi token dan mengembalikan respons berhasil tanpa mencabut token; token tetap berlaku sampai kedaluwarsa. Client sebaiknya menghapus token lokal setelah logout.
