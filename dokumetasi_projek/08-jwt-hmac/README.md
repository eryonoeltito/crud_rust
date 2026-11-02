# JWT HMAC

Panduan ini membahas JWT yang dipakai proyek, dari secret sampai request yang dilindungi. Baca berurutan; setiap langkah mengikuti file dan fungsi yang benar-benar ada di source.

## 1. Gambaran singkat

JWT adalah token yang server buat setelah login berhasil. Proyek menandatanganinya dengan **HMAC SHA-256 (HS256)**. Server memakai satu secret yang sama untuk membuat dan memeriksa tanda tangan. Client hanya menyimpan dan mengirim token; client tidak perlu mengetahui secret.

Token berisi claim `sub` (username) dan `exp` (waktu kedaluwarsa). Tanda tangan membuat perubahan pada token dapat diketahui. Isi token tidak dienkripsi, jadi jangan masukkan password atau informasi rahasia ke dalam claim.

## 2. Siapkan secret

`src/main.rs` membaca `JWT_SECRET` dari environment. Server berhenti saat startup jika nilainya tidak ada atau panjangnya kurang dari 32 byte. Buat nilai acak, misalnya dengan `openssl rand -hex 32`, kemudian simpan hasilnya sebagai `JWT_SECRET=...` di `.env`. Jangan commit nilai rahasia tersebut.

## 3. Membuat token saat login

Urutannya ada di `src/controllers/auth_controller.rs` dan `src/auth.rs`:

1. Handler login memeriksa username dan password lewat `User::cek_login`. Password diverifikasi terhadap hash Argon2 yang disimpan di database.
2. Jika cocok, handler memanggil `buat_token(username, secret)`.
3. `buat_token` menghitung waktu sekarang dalam detik Unix, lalu menambah 1 jam untuk nilai `exp`.
4. Fungsi membentuk claim `{ sub, exp }` dan meminta crate `jsonwebtoken` menandatanganinya dengan `HS256` serta `JWT_SECRET`.
5. Handler mengirim token di JSON response. Client menyimpan token dan mengirimkannya pada request berikutnya.

Response login memiliki bentuk seperti ini:

```json
{
  "status": "success",
  "token_type": "Bearer",
  "token": "<jwt>",
  "expires_in": 3600
}
```

Ganti `<jwt>` dengan nilai `token` yang benar-benar diterima. Token ini kredensial aktif: jangan bagikan atau catat ke log.

## 4. Mengirim token

Tambahkan token ke header `Authorization` untuk route barang:

```http
Authorization: Bearer <jwt>
```

`Bearer` menyatakan bahwa client mengirim token sebagai bukti akses. Jangan sertakan tanda kurung sudut pada nilai sebenarnya.

## 5. Memeriksa token pada middleware

`src/middleware/auth_middleware.rs` menjalankan pemeriksaan sebelum handler barang:

1. Ambil nilai header `Authorization`.
2. Pastikan formatnya diawali `Bearer `, lalu ambil token setelahnya.
3. Panggil `validasi_token(token, secret)` dari `src/auth.rs`.
4. Fungsi validasi hanya menerima algoritma `HS256`, meminta claim `exp` dan `sub`, lalu memeriksa signature dan kedaluwarsa.
5. Jika valid, middleware menjalankan handler berikutnya dengan `next.run(request)`. Jika tidak ada atau tidak valid, API mengembalikan `401 Unauthorized`.

## 6. Logout dan batas JWT stateless

Endpoint `POST /logout` memberi respons agar client menghapus token. Server tidak menyimpan daftar token aktif, jadi logout tidak bisa membatalkan token yang sudah dibuat. Token yang masih dipegang orang lain tetap berlaku sampai 1 jamnya habis.

## Peta file

| File | Tugas |
|---|---|
| `src/main.rs` | Membaca dan memeriksa secret, lalu memasangnya sebagai `Extension` Axum. |
| `src/auth.rs` | Membuat claim/token dan memvalidasi token HS256. |
| `src/controllers/auth_controller.rs` | Memeriksa login, mengembalikan token, dan menangani logout. |
| `src/middleware/auth_middleware.rs` | Mengambil Bearer token dan melindungi route barang. |

Ikuti alur belajar ini: mulai dari `main.rs` untuk memahami dari mana secret datang, lanjut ke `buat_token`, lalu ikuti token dari response login ke middleware dan `validasi_token`.
