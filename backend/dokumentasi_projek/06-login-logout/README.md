# 06 — Login dan Logout

Autentikasi membuktikan identitas pengguna sebelum route barang dan riwayat dipakai. Route ada di `src/routes/auth_routes.rs`, controller di `src/controllers/auth_controller.rs`, dan query akun di `src/models/user.rs`.

## Registrasi

`POST /api/auth/register` menerima:

```json
{"username":"andi","password":"rahasia"}
```

Username yang hanya berisi spasi atau password kosong menghasilkan `400 Bad Request`. Password dibuat menjadi hash Argon2 dengan salt acak dari `OsRng`, lalu disimpan melalui query berparameter. Username yang sudah ada menghasilkan `409 Conflict`; berhasil mendaftar menghasilkan `201 Created`:

```json
{"status":"success","message":"Pendaftaran berhasil"}
```

Query registrasi hanya memasukkan username dan password yang sudah di-hash; kode tidak menetapkan `aktif`, `role`, atau `gudang_id`. Migration memberi `aktif` default `FALSE`, sedangkan `role` dan `gudang_id` boleh `NULL`. Constraint akun mengatur kombinasinya: akun pending harus belum aktif dan belum memiliki role maupun gudang; admin yang aktif tidak memiliki `gudang_id`; petugas gudang yang aktif wajib memiliki `gudang_id`.

Karena akun baru berstatus pending, akun tersebut belum dapat langsung login. Akun perlu diaktifkan dan diberi role serta gudang yang sesuai melalui proses administrasi di luar endpoint register; API saat ini tidak menyediakan endpoint aktivasi. Setelah aktif, admin dapat login tanpa gudang, sedangkan petugas harus sudah memiliki gudang tugas.

## Login

`POST /api/auth/login` memakai body username dan password. Model mengambil `id`, hash password, `aktif`, dan `role`, lalu memverifikasi password dengan Argon2.

| Kondisi | Response |
|---|---|
| Username tidak ada, password salah, atau role bukan `admin`/`petugas_gudang` | `401` — `Username atau password salah` |
| Password benar tetapi `aktif = false` | `403` — `Akun tidak aktif. Hubungi administrator.` |
| Password benar, akun aktif, role diizinkan | `200` dan JWT |
| Query/koneksi gagal | `500` |

Response sukses memuat `status`, `message`, `token_type: "Bearer"`, `token`, dan `expires_in: 1800`. Token dibuat dengan HS256 dan berlaku 30 menit. Detail claim dan validasinya ada di [08 — JWT HMAC](../08-jwt-hmac/README.md).

## Logout

`POST /api/auth/logout` memerlukan header Bearer yang valid, tetapi tidak memakai middleware database. Handler memvalidasi JWT lalu mengembalikan:

```json
{"status":"success","message":"Logout berhasil"}
```

Logout tidak menyimpan daftar pencabutan token. JWT tetap dapat dipakai sampai `exp` tercapai, selama akun masih lolos pemeriksaan middleware pada route terlindungi. Client harus menghapus token lokal.

## Perbedaan autentikasi dan otorisasi

Autentikasi menjawab “siapa pengguna ini?” melalui JWT dan akun database. Otorisasi menjawab “apa yang boleh dilakukan?” melalui role dan `gudang_id` terbaru dari database. Claim `role` di JWT bukan sumber hak akses.
