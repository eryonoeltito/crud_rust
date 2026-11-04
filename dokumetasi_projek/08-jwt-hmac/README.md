# JWT HMAC

JWT adalah token yang server buat setelah login berhasil. Proyek menandatanganinya dengan **HMAC SHA-256 (HS256)**. Server memakai secret yang sama untuk membuat dan memeriksa tanda tangan. Isi JWT tidak dienkripsi dan dapat dibaca, jadi jangan masukkan password atau informasi rahasia ke dalam claim.

## Claim

Token memuat claim `sub` (ID pengguna dalam bentuk teks), `exp` (waktu kedaluwarsa), dan `jti` (identitas acak token). Tanda tangan HS256 membuat perubahan pada token dapat diketahui. Token berlaku selama satu jam.

## Membuat token saat login

1. Handler login memeriksa username dan password melalui `User::cek_login`. Password diverifikasi terhadap hash Argon2 di database.
2. Model mengambil ID, hash password, dan status aktif terbaru dari database, lalu memverifikasi password memakai Argon2. Akun nonaktif tidak memperoleh token.
3. Jika login diizinkan, handler memanggil `buat_token(user_id, secret)`.
4. `buat_token` mengisi `sub` dengan ID pengguna, membuat `jti` acak, dan menetapkan `exp` satu jam ke depan.
5. Crate `jsonwebtoken` menandatangani claim dengan algoritma HS256 dan `JWT_SECRET`.
6. Handler mengirim token melalui response JSON. Client mengirimkannya pada request berikutnya.

Contoh bentuk response login:

```json
{
  "status": "success",
  "token_type": "Bearer",
  "token": "<jwt>",
  "expires_in": 3600
}
```

Jangan membagikan atau mencatat token yang sebenarnya karena token adalah kredensial aktif.

## Mengirim dan memeriksa token

Kirim token melalui header berikut untuk route barang:

```http
Authorization: Bearer <jwt>
```

Middleware `wajib_login` membaca header, mengambil token dari format Bearer, lalu memanggil `validasi_token`. Validasi hanya menerima HS256 serta memeriksa signature dan claim wajib `exp`, `sub`, dan `jti`. `sub` harus berisi ID pengguna dan `jti` tidak boleh kosong. Pemeriksaan `exp` menggunakan `leeway = 0`, sehingga tidak ada toleransi waktu tambahan setelah token kedaluwarsa. Token tanpa header Bearer yang benar, signature tidak valid, claim wajib yang hilang, atau token kedaluwarsa ditolak dengan `401 Unauthorized`.

## Logout dan masa berlaku

Endpoint `POST /logout` memvalidasi signature, `exp`, `sub`, dan `jti`, lalu menyimpan `jti` beserta `expires_at` dari claim `exp` di `public.revoked_tokens`. Query memakai `ON CONFLICT (jti) DO NOTHING`, jadi logout berulang tetap berhasil tanpa baris duplikat. Validasi khusus logout tidak menolak token hanya karena sudah tercatat revoked, agar logout dapat diulang. Token lama tanpa `jti` ditolak; pengguna harus login kembali.

Middleware route barang belum memeriksa tabel revoked token. Jadi pencatatan logout sudah dilakukan server, tetapi penolakan token yang dicabut pada endpoint barang menunggu Tahap 2. Client sebaiknya tetap menghapus token lokal setelah logout.

## Status pengujian

Hasil berikut adalah catatan pengujian manual terdahulu, sebelum pencatatan revocation pada server diterapkan:

| Pengujian | Hasil |
|---|---|
| Login dengan akun valid | `200 OK` |
| GET `/barang` dengan token valid | `200 OK` |
| POST `/logout` dengan token | `200 OK` |
| GET `/barang` dengan token yang sama setelah logout | `200 OK` |
| GET `/barang` tanpa token | `401 Unauthorized` |

Hasil tersebut menggambarkan perilaku lama. Saat ini logout mencatat token di server, tetapi hasil `GET /barang` setelah logout belum berubah karena middleware Tahap 2 belum memeriksa tabel pencabutan. Pengujian akses token revoked setelah Tahap 2 belum dilakukan.

Pemeriksaan `cargo fmt --check`, `cargo check`, dan `cargo test` berhasil. Unit test terakhir melaporkan 9 lulus dan 2 test integrasi database diabaikan karena memerlukan `DATABASE_URL_TEST` yang menunjuk database uji terpisah dengan skema hasil migration. Pengujian integrasi login aktif/nonaktif dan logout berulang belum dijalankan.

## Peta file

| File | Tugas |
|---|---|
| `src/auth.rs` | Membuat claim dan token HS256 serta memvalidasi token. |
| `src/controllers/auth_controller.rs` | Memeriksa login, mengembalikan token, dan menangani logout. |
| `src/middleware/auth_middleware.rs` | Membaca Bearer token dan melindungi route barang. |
| `src/routes/barang_routes.rs` | Memasang middleware autentikasi pada seluruh route barang. |
