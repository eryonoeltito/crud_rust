# 07 — JWT HMAC dan Middleware

Nama folder lama masih memuat kata `session-cookie`, tetapi implementasi saat ini tidak memakai session atau cookie. Autentikasi menggunakan JWT pada header Bearer.

## Siklus request

```text
Authorization: Bearer <token>
        ↓
wajib_login → validasi HS256 dan claim → ambil akun terbaru dari database
        ↓
request extensions: AkunTerverifikasi → controller
```

`wajib_login` berada di `src/middleware/auth_middleware.rs` dan dipasang pada seluruh route barang serta riwayat. Token yang hilang, format Bearer salah, signature salah, claim wajib tidak lengkap, role tidak diizinkan, atau token kedaluwarsa menghasilkan `401 Unauthorized` dengan pesan umum.

## Pemeriksaan database

Setelah JWT valid, middleware mengubah `sub` menjadi ID integer, mengambil koneksi dengan `pool.acquire()`, lalu menjalankan `User::ambil_akun_terverifikasi(&mut koneksi, id)`. Query hanya menerima akun dengan `aktif = TRUE` dan kombinasi berikut:

- `role = 'admin'` serta `gudang_id IS NULL`; atau
- `role = 'petugas_gudang'` serta `gudang_id IS NOT NULL`.

Akun yang tidak ditemukan, tidak aktif, atau kombinasi role/gudangnya tidak valid menghasilkan `401`. Kegagalan membuka koneksi atau query menghasilkan `500`. Data terbaru disimpan sebagai `AkunTerverifikasi { id, role, gudang_id }` pada request extensions.

Karena role dan gudang diambil ulang dari database, perubahan akun berlaku pada request berikutnya meskipun JWT masih memiliki claim lama. Middleware tidak mempercayai role atau gudang dari body request.

## Secret dan logout

`JWT_SECRET` dibaca saat startup dan harus memiliki panjang minimal 32 byte. Nilai rahasia tidak boleh ditulis di dokumentasi. `POST /logout` memvalidasi token di handler dan tidak mencabut token di server; client perlu menghapus token lokal.
