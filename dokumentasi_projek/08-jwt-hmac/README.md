# 08 — JWT HMAC

JWT adalah token yang dibuat setelah login berhasil. Proyek memakai HMAC SHA-256 (HS256). JWT ditandatangani, bukan dienkripsi; payload dapat dibaca sehingga tidak boleh berisi password atau secret.

## Claim

Token memuat `sub` (ID pengguna sebagai teks), `role` (role saat token diterbitkan), `iat` (waktu diterbitkan), dan `exp` (kedaluwarsa). `exp` ditetapkan 1.800 detik setelah `iat`, yaitu 30 menit. Role yang dapat diterbitkan hanya `admin` dan `petugas_gudang`.

Contoh payload, bukan token nyata:

```json
{"sub":"12","role":"admin","iat":1760000000,"exp":1760001800}
```

`JWT_SECRET` dibaca dari environment dan harus minimal 32 byte. Secret yang sama dipakai untuk membuat dan memeriksa signature.

## Validasi token

Fungsi `validasi_token` memakai algoritma HS256, mewajibkan claim `exp`, `sub`, `iat`, dan `role`, serta menetapkan `leeway = 0`. Fungsi juga memastikan `sub` dapat diparse sebagai `i64` dan role termasuk dua role yang diizinkan. Claim `iat` wajib ada, tetapi tidak dipakai sebagai mekanisme pencabutan token.

Middleware kemudian memeriksa akun terbaru di database. Role pada JWT hanya catatan saat token dibuat; otorisasi memakai role dan `gudang_id` dari `AkunTerverifikasi` hasil query database.

## Login, penggunaan, dan logout

Login berhasil mengembalikan bentuk berikut:

```json
{
  "status":"success",
  "message":"Login berhasil",
  "token_type":"Bearer",
  "token":"<jwt>",
  "expires_in":1800
}
```

Kirim token seperti ini:

```http
Authorization: Bearer <jwt>
```

Logout hanya memvalidasi token dan mengembalikan sukses. Tidak ada revocation list; token tetap berlaku sampai kedaluwarsa atau sampai middleware menolak akun yang sudah tidak aktif/valid.

## Peta file

| File | Tugas |
|---|---|
| `src/auth.rs` | Membuat dan memvalidasi JWT HS256 |
| `src/controllers/auth_controller.rs` | Registrasi, login, dan logout |
| `src/middleware/auth_middleware.rs` | Validasi token dan akun terbaru |
| `src/models/user.rs` | Hash/verifikasi password dan query akun |


## Status pengujian
Dokumentasi ini menjelaskan perilaku berdasarkan pembacaan source code. Tidak ada hasil pengujian runtime atau laporan `cargo test` yang diverifikasi dalam pekerjaan ini. Pengujian yang perlu dilakukan meliputi login valid/salah, token kedaluwarsa, token tanpa Bearer, akun nonaktif, perubahan role/gudang, akses admin, dan pembatasan gudang petugas.
