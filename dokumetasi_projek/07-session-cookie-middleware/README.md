# 07 — JWT HMAC dan Middleware

API menggunakan JWT bertanda tangan HMAC SHA-256 (HS256) sebagai token autentikasi. Token membawa ID pengguna sebagai teks pada claim `sub`, waktu kedaluwarsa pada claim `exp`, dan identitas unik token pada claim `jti`. Middleware memverifikasi signature dan masa berlaku token sebelum meneruskan request ke route barang.

## Konfigurasi secret

Secret JWT dibaca dari environment variable `JWT_SECRET`. Nilainya harus rahasia dan minimal 32 byte. Simpan di `.env` atau pengelola secret deployment; jangan commit nilai secret yang sebenarnya. Untuk membuat secret acak, jalankan `openssl rand -hex 32`, lalu masukkan hasilnya ke `.env` sebagai `JWT_SECRET`.

## Siklus request

```text
Login berhasil → server mengirim JWT HS256 di response JSON
Request berikutnya + Authorization: Bearer <token> → middleware memverifikasi token
```

Token berlaku selama 1 jam. Kirim token pada header `Authorization` untuk setiap request yang memerlukan login:

```http
Authorization: Bearer <token>
```

`wajib_login` berada di `src/middleware/auth_middleware.rs` dan melindungi semua route barang. Token yang hilang, rusak, memiliki signature salah, atau kedaluwarsa ditolak dengan `401 Unauthorized`. Handler `POST /logout` memvalidasi token lalu mencatat `jti` dan `exp` di `public.revoked_tokens`; logout berulang tidak membuat baris ganda.

Middleware saat ini belum memeriksa `public.revoked_tokens`. Jadi pencatatan logout telah dilakukan server, tetapi route barang belum menolak token yang dicabut. Pemeriksaan revocation pada middleware akan diterapkan pada Tahap 2. Client tetap sebaiknya menghapus token lokal setelah logout.
