# JWT HMAC

JWT adalah token yang server buat setelah login berhasil. Proyek menandatanganinya dengan **HMAC SHA-256 (HS256)**. Server memakai secret yang sama untuk membuat dan memeriksa tanda tangan. Isi JWT tidak dienkripsi dan dapat dibaca, jadi jangan masukkan password atau informasi rahasia ke dalam claim.

## Claim

Token memuat claim `sub` (ID pengguna dalam bentuk teks), `iat` (waktu diterbitkan), dan `exp` (waktu kedaluwarsa). Tanda tangan HS256 membuat perubahan pada token dapat diketahui. Token berlaku selama 30 menit. Claim `iat` hanya mencatat waktu penerbitan dan tidak digunakan untuk mencabut atau menolak token.

## Membuat token saat login

1. Handler login memeriksa username dan password melalui `User::cek_login`. Password diverifikasi terhadap hash Argon2 di database.
2. Model mengambil ID, hash password, dan status aktif terbaru dari database, lalu memverifikasi password memakai Argon2. Akun nonaktif tidak memperoleh token.
3. Jika login diizinkan, handler memanggil `buat_token(user_id, secret)`.
4. `buat_token` mengisi `sub` dengan ID pengguna, `iat` dengan waktu saat ini, dan menetapkan `exp` 1800 detik setelah `iat`.
5. Crate `jsonwebtoken` menandatangani claim dengan algoritma HS256 dan `JWT_SECRET`.
6. Handler mengirim token melalui response JSON. Client mengirimkannya pada request berikutnya.

Contoh bentuk response login:

```json
{
  "status": "success",
  "token_type": "Bearer",
  "token": "<jwt>",
  "expires_in": 1800
}
```

Jangan membagikan atau mencatat token yang sebenarnya karena token adalah kredensial aktif.

## Mengirim dan memeriksa token

Kirim token melalui header berikut untuk route barang:

```http
Authorization: Bearer <jwt>
```

Middleware `wajib_login` membaca header, mengambil token dari format Bearer, lalu memanggil `validasi_token`. Validasi hanya menerima HS256 serta memeriksa signature, format claim `sub`, `iat`, dan `exp`, serta kedaluwarsa. `sub` harus berisi ID pengguna. Pemeriksaan `exp` menggunakan `leeway = 0`, sehingga tidak ada toleransi waktu tambahan setelah token kedaluwarsa. Nilai `iat` tidak diperiksa untuk memutuskan apakah token boleh digunakan. Token tanpa header Bearer yang benar, signature tidak valid, claim wajib yang hilang, atau token kedaluwarsa ditolak dengan `401 Unauthorized`.

Setelah token lolos, middleware membaca akun aktif berdasarkan ID `sub` dari `public.users`. Akun yang tidak ditemukan, nonaktif, atau tidak memiliki kombinasi role dan gudang yang diizinkan ditolak dengan `401 Unauthorized`. Nilai `id`, `role`, dan `gudang_id` terbaru disimpan sebagai `AkunTerverifikasi` pada request extensions agar dapat digunakan handler. Gangguan koneksi atau query database menghasilkan `500 Internal Server Error`, bukan dianggap sebagai token tidak valid. Role dan gudang tidak dipercaya dari claim JWT atau JSON request.

## Logout dan masa berlaku

Endpoint `POST /logout` memvalidasi Bearer token, lalu mengembalikan respons berhasil tanpa menyimpan pencabutan token. Token tetap dapat digunakan sampai claim `exp` tercapai; logout tidak membuat JWT langsung tidak berlaku. Client sebaiknya menghapus token lokal setelah logout.

Middleware melindungi route terlindungi dan memeriksa akun aktif serta role/gudang terbaru dari database. Route register, login, dan logout tetap berada pada router terpisah. Permintaan logout berulang tetap berhasil selama token belum kedaluwarsa.

## Status pengujian

Hasil berikut adalah catatan pengujian manual terdahulu, sebelum middleware Tahap 2 memeriksa revocation:

| Pengujian | Hasil |
|---|---|
| Login dengan akun valid | `200 OK` |
| GET `/barang` dengan token valid | `200 OK` |
| POST `/logout` dengan token | `200 OK` |
| GET `/barang` dengan token yang sama setelah logout | `200 OK` |
| GET `/barang` tanpa token | `401 Unauthorized` |

Tabel tersebut adalah catatan pengujian manual terdahulu. Hasil token masih dapat mengakses route setelah logout sesuai perilaku saat ini, karena logout tidak mencabut token. Pengujian manual untuk masa berlaku baru 30 menit belum dicatat.

Pada pemeriksaan Tahap 2, `cargo fmt --check`, `cargo check`, dan `cargo test` berhasil. Test suite saat itu melaporkan 11 lulus dan 2 test integrasi database diabaikan karena memerlukan `DATABASE_URL_TEST` yang menunjuk database uji terpisah dengan skema hasil migration. Pengujian HTTP middleware dengan database—termasuk akun nonaktif dan kegagalan database—belum dijalankan.

## Peta file

| File | Tugas |
|---|---|
| `src/auth.rs` | Membuat claim dan token HS256 serta memvalidasi token. |
| `src/controllers/auth_controller.rs` | Memeriksa login, mengembalikan token, dan menangani logout. |
| `src/middleware/auth_middleware.rs` | Memvalidasi Bearer token, memeriksa akun terbaru, lalu meneruskan `AkunTerverifikasi` ke handler. |
| `src/models/user.rs` | Memeriksa login serta mengambil data akun aktif terbaru untuk middleware. |
| `src/routes/barang_routes.rs` | Memasang middleware autentikasi pada seluruh route barang. |
