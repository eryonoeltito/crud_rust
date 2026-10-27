# 11 — Penjelasan kode `src/models/user.rs`

Dokumen ini menjelaskan fungsi setiap bagian kode model `User` untuk pendaftaran dan pemeriksaan login. Nama file sumber: `src/models/user.rs`.

## Kode sumber dengan penjelasan

```rust
use sqlx::PgPool;
```
- `use` mengimpor nama agar bisa digunakan di file ini.
- `sqlx::PgPool` adalah pool koneksi ke PostgreSQL. Pool mengelola koneksi agar handler tidak perlu membuka koneksi baru secara manual untuk setiap query.

```rust
pub struct User;
```
- `struct User;` mendeklarasikan unit struct bernama `User`, tanpa field.
- `pub` membuat nama `User` dapat diakses dari modul lain.
- Pada pola kode ini, `User` dipakai sebagai tempat mengelompokkan fungsi terkait pengguna; tidak ada objek `User` yang perlu dibuat.

```rust
impl User {
```
- `impl User` menuliskan fungsi yang terkait dengan tipe `User`.
- Kurung kurawal membuka blok implementasi.

## Fungsi `register`

```rust
    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, sqlx::Error> {
```
- Fungsi asinkron ini menerima pool database dan kredensial baru.
- `Result<bool, sqlx::Error>` berarti `Ok(true)` jika akun berhasil dibuat, `Ok(false)` jika username sudah ada, atau `Err(...)` jika query database gagal.

```rust
        let username_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;
```
- `SELECT EXISTS` memeriksa apakah tabel `users` sudah memiliki username tersebut.
- `.bind(username)` mengikat nilai ke placeholder `$1` sehingga nilainya tidak digabung ke SQL secara langsung.
- `.fetch_one(pool).await?` menjalankan query dan meneruskan error SQLx ke pemanggil jika query gagal.

```rust
        if username_exists {
            return Ok(false);
        }
```
- Jika username sudah terdaftar, fungsi berhenti tanpa menjalankan `INSERT` dan memberi hasil `Ok(false)`.

```rust
        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(username)
            .bind(password)
            .execute(pool)
            .await?;

        Ok(true)
    }
```
- `INSERT` menambahkan username dan password ke tabel `users`.
- `.bind(...)` mengikat kedua nilai ke placeholder `$1` dan `$2`.
- `.execute(pool).await?` menjalankan perintah tanpa mengambil baris hasil dan meneruskan error jika gagal.
- Setelah insert berhasil, `Ok(true)` menandakan akun dibuat.

Pemeriksaan username dilakukan sebelum insert. Jika database tidak memiliki batasan unik pada kolom `username`, dua request bersamaan masih dapat melewati pemeriksaan ini dan membuat username yang sama.

```rust
    pub async fn cek_login(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, sqlx::Error> {
```
- `pub`: fungsi dapat dipanggil dari modul lain.
- `async`: fungsi bisa menunggu operasi I/O seperti query database tanpa memblokir thread secara langsung.
- `fn cek_login`: nama fungsi pemeriksaan login.
- `pool: &PgPool`: meminjam referensi pool PostgreSQL.
- `username: &str` dan `password: &str`: menerima referensi teks untuk kredensial.
- `-> Result<bool, sqlx::Error>`: jika query berhasil, hasilnya `Ok(true)` atau `Ok(false)`; jika query database gagal, hasilnya `Err(sqlx::Error)`.

```rust
        let user = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE username = $1
                  AND password = $2
            )"
        )
```
- `let user` membuat variabel untuk menyimpan hasil query.
- `query_scalar::<_, bool>` meminta SQLx mengambil satu nilai skalar bertipe `bool`.
- `SELECT EXISTS (...)` menghasilkan `true` jika query di dalamnya menemukan baris yang cocok, atau `false` jika tidak.
- `SELECT 1 FROM users` memeriksa baris pada tabel `users`; angka `1` di sini hanya nilai penanda, bukan ID pengguna.
- `WHERE username = $1 AND password = $2` mencari baris yang username dan password-nya cocok dengan parameter.
- `$1` dan `$2` adalah placeholder SQL yang nilainya diikat lewat `.bind(...)`, bukan digabung langsung ke teks SQL.

```rust
        .bind(username)
        .bind(password)
```
- `.bind(username)` mengikat nilai username ke parameter `$1`.
- `.bind(password)` mengikat nilai password ke parameter `$2`.
- Mengikat parameter membantu mencegah SQL injection pada nilai kredensial.

```rust
        .fetch_one(pool)
        .await?;
```
- `.fetch_one(pool)` menjalankan query dan mengambil satu hasil skalar.
- `.await` menunggu query selesai.
- `?` meneruskan error ke pemanggil jika query gagal; jika berhasil, nilai hasilnya disimpan ke `user`.

```rust
        Ok(user)
    }
}
```
- `Ok(user)` membungkus nilai boolean dalam `Result` sukses.
- Kurung kurawal menutup fungsi dan blok `impl`.

## Alur singkat

1. Untuk pendaftaran, controller memberikan pool, username, dan password ke `User::register`.
2. Model memeriksa apakah username sudah dipakai, lalu memasukkan akun baru jika belum.
3. Untuk login, controller memberikan kredensial ke `User::cek_login` dan model memeriksa tabel `users`.
4. Pemeriksaan login menghasilkan `Ok(true)` jika pasangan kredensial cocok dan `Ok(false)` jika tidak cocok.
5. Jika query gagal, fungsi mengembalikan error SQLx.

## Catatan keamanan

Source yang dijelaskan di sini membandingkan `password` secara langsung dengan kolom database. Itu berarti password saat ini disimpan sebagai teks biasa. Gunakan hanya untuk demo lokal; aplikasi nyata harus menyimpan hash password yang dibuat dengan algoritma password hashing seperti Argon2.
