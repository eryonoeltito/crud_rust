# JWT HMAC

JWT adalah token yang server buat setelah login berhasil. Proyek menandatanganinya dengan **HMAC SHA-256 (HS256)**. Server memakai secret yang sama untuk membuat dan memeriksa tanda tangan. Isi JWT tidak dienkripsi dan dapat dibaca, jadi jangan masukkan password atau informasi rahasia ke dalam claim.

## Claim

Token memuat claim `sub` (username) dan `exp` (waktu kedaluwarsa). Tanda tangan HS256 membuat perubahan pada token dapat diketahui. Token berlaku selama satu jam.

## Membuat token saat login

1. Handler login memeriksa username dan password melalui `User::cek_login`. Password diverifikasi terhadap hash Argon2 di database.
2. Jika cocok, handler memanggil `buat_token(username, secret)`.
3. `buat_token` mengisi `sub` dengan username, menghitung waktu sekarang, lalu menetapkan `exp` satu jam ke depan.
4. Crate `jsonwebtoken` menandatangani claim dengan algoritma HS256 dan `JWT_SECRET`.
5. Handler mengirim token melalui response JSON. Client mengirimkannya pada request berikutnya.

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

Middleware `wajib_login` membaca header, mengambil token dari format Bearer, lalu memanggil `validasi_token`. Validasi hanya menerima HS256 serta memeriksa signature dan claim wajib `exp` serta `sub`. Pemeriksaan `exp` menggunakan `leeway = 0`, sehingga tidak ada toleransi waktu tambahan setelah token kedaluwarsa. Token tanpa header Bearer yang benar, signature tidak valid, atau sudah kedaluwarsa ditolak dengan `401 Unauthorized`. Token yang valid diteruskan ke handler.

## Logout dan masa berlaku

Endpoint `POST /logout` mengembalikan respons logout berhasil dan meminta client menghapus token dari penyimpanan lokal. Server tidak membatalkan token. Token yang sama tetap dapat digunakan sampai waktu `exp` terlewati, selama signature HS256-nya valid.

## Status pengujian

Hasil pengujian runtime manual menggunakan curl:

| Pengujian | Hasil |
|---|---|
| Login dengan akun valid | `200 OK` |
| GET `/barang` dengan token valid | `200 OK` |
| POST `/logout` dengan token | `200 OK` |
| GET `/barang` dengan token yang sama setelah logout | `200 OK` |
| GET `/barang` tanpa token | `401 Unauthorized` |

Hasil ini menunjukkan logout meminta client menghapus token, tetapi token yang sama tetap berlaku di server. Middleware tetap menolak request tanpa token. Pengujian kedaluwarsa token belum dilakukan secara runtime; middleware tetap memeriksa claim `exp`.

Pemeriksaan statis `cargo fmt --check`, `cargo check`, dan `cargo test` berhasil. `cargo test` melaporkan 0 automated test; hasil tersebut bukan pengujian otomatis untuk logout atau kedaluwarsa JWT.

## Peta file

| File | Tugas |
|---|---|
| `src/auth.rs` | Membuat claim dan token HS256 serta memvalidasi token. |
| `src/controllers/auth_controller.rs` | Memeriksa login, mengembalikan token, dan menangani logout. |
| `src/middleware/auth_middleware.rs` | Membaca Bearer token dan melindungi route barang. |
| `src/routes/barang_routes.rs` | Memasang middleware autentikasi pada seluruh route barang. |
