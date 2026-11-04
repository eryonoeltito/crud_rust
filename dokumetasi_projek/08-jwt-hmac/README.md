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

Middleware `wajib_login` membaca header, mengambil token dari format Bearer, lalu memanggil `validasi_token`. Validasi hanya menerima HS256 serta memeriksa signature dan claim wajib `exp`, `sub`, dan `jti`. `sub` harus berisi ID pengguna dan `jti` tidak boleh kosong. Pemeriksaan `exp` menggunakan `leeway = 0`, sehingga tidak ada toleransi waktu tambahan setelah token kedaluwarsa. Token tanpa header Bearer yang benar, signature tidak valid, claim wajib yang hilang, token kedaluwarsa, atau token yang tercatat di `public.revoked_tokens` ditolak dengan `401 Unauthorized`.

Setelah token lolos, middleware membaca akun aktif berdasarkan ID `sub` dari `public.users`. Akun yang tidak ditemukan, nonaktif, atau tidak memiliki kombinasi role dan gudang yang diizinkan ditolak dengan `401 Unauthorized`. Nilai `id`, `role`, dan `gudang_id` terbaru disimpan sebagai `AkunTerverifikasi` pada request extensions agar dapat digunakan handler. Gangguan koneksi atau query database menghasilkan `500 Internal Server Error`, bukan dianggap sebagai token tidak valid. Role dan gudang tidak dipercaya dari claim JWT atau JSON request.

## Logout dan masa berlaku

Endpoint `POST /logout` memvalidasi signature, `exp`, `sub`, dan `jti`, lalu menyimpan `jti` beserta `expires_at` dari claim `exp` di `public.revoked_tokens`. Query memakai `ON CONFLICT (jti) DO NOTHING`, jadi logout berulang tetap berhasil tanpa baris duplikat. Validasi khusus logout tidak menolak token hanya karena sudah tercatat revoked, agar logout dapat diulang. Token lama tanpa `jti` ditolak; pengguna harus login kembali.

Middleware melindungi seluruh route barang dan memeriksa tabel revoked token. Route register, login, dan logout tetap berada pada router terpisah. Logout tidak memeriksa status revoked saat memvalidasi token, sehingga permintaan logout berulang tetap berhasil selama token belum kedaluwarsa. Client sebaiknya tetap menghapus token lokal setelah logout.

## Status pengujian

Hasil berikut adalah catatan pengujian manual terdahulu, sebelum middleware Tahap 2 memeriksa revocation:

| Pengujian | Hasil |
|---|---|
| Login dengan akun valid | `200 OK` |
| GET `/barang` dengan token valid | `200 OK` |
| POST `/logout` dengan token | `200 OK` |
| GET `/barang` dengan token yang sama setelah logout | `200 OK` |
| GET `/barang` tanpa token | `401 Unauthorized` |

Hasil tersebut menggambarkan perilaku sebelum Tahap 2 dan bukan hasil pengujian implementasi terkini. Setelah Tahap 2, token yang sudah dicabut seharusnya menerima `401` pada route barang. Pengujian manual terkini untuk perilaku ini belum dicatat.

Pada pemeriksaan Tahap 2, `cargo fmt --check`, `cargo check`, dan `cargo test` berhasil. Test suite melaporkan 11 lulus dan 2 test integrasi database diabaikan karena memerlukan `DATABASE_URL_TEST` yang menunjuk database uji terpisah dengan skema hasil migration. Pengujian HTTP middleware dengan database—termasuk token revoked, akun nonaktif, dan kegagalan database—belum dijalankan.

## Peta file

| File | Tugas |
|---|---|
| `src/auth.rs` | Membuat claim dan token HS256 serta memvalidasi token. |
| `src/controllers/auth_controller.rs` | Memeriksa login, mengembalikan token, dan menangani logout. |
| `src/middleware/auth_middleware.rs` | Memvalidasi Bearer token, memeriksa pencabutan dan akun terbaru, lalu meneruskan `AkunTerverifikasi` ke handler. |
| `src/models/revoked_token.rs` | Menyimpan pencabutan token secara idempotent dan memeriksa status pencabutan. |
| `src/models/user.rs` | Memeriksa login serta mengambil data akun aktif terbaru untuk middleware. |
| `src/routes/barang_routes.rs` | Memasang middleware autentikasi pada seluruh route barang. |
